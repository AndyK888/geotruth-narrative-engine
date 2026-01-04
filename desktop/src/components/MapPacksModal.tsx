import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './MapPacksModal.css';

const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

interface RegionInfo {
    id: string;
    name: string;
    size_mb: number;
    downloaded: boolean;
    last_updated: string | null;
    poi_count: number;
    bounds: [number, number, number, number];
}

interface DownloadProgress {
    region_id: string;
    bytes_downloaded: number;
    total_bytes: number;
    progress_percent: number;
    status: string;
}

interface MapPacksModalProps {
    isOpen: boolean;
    onClose: () => void;
    onStatusChange: (downloaded: number, total: number) => void;
}

export function MapPacksModal({ isOpen, onClose, onStatusChange }: MapPacksModalProps) {
    const [regions, setRegions] = useState<RegionInfo[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [downloadProgress, setDownloadProgress] = useState<DownloadProgress | null>(null);
    const [activeDownload, setActiveDownload] = useState<string | null>(null);
    const [availableRegions, setAvailableRegions] = useState<RegionInfo[]>([]);
    const [selectedRegionId, setSelectedRegionId] = useState<string>('');

    useEffect(() => {
        if (isOpen) {
            loadRegions();
        }
    }, [isOpen]);

    useEffect(() => {
        let interval: number | null = null;

        if (activeDownload) {
            interval = window.setInterval(async () => {
                try {
                    const progress = await invoke<DownloadProgress | null>('get_download_progress');
                    setDownloadProgress(progress);

                    if (!progress) {
                        // Download complete
                        setActiveDownload(null);
                        await loadRegions();
                    }
                } catch (e) {
                    console.error('Failed to get progress:', e);
                }
            }, 500);
        }

        return () => {
            if (interval) clearInterval(interval);
        };
    }, [activeDownload]);

    const loadRegions = async () => {
        setLoading(true);
        setError(null);
        try {
            const current: RegionInfo[] = await invoke('get_map_regions');
            setRegions(current);

            // Also load available regions for the dropdown
            const available: RegionInfo[] = await invoke('get_available_regions');
            setAvailableRegions(available);

            const downloaded = current.filter(r => r.downloaded).length;
            onStatusChange(downloaded, current.length);
        } catch (e) {
            setError(`Failed to load regions: ${e}`);
            console.error('Failed to load regions:', e);
        } finally {
            setLoading(false);
        }
    };

    const handleDownload = async (regionId: string) => {
        setActiveDownload(regionId);
        setError(null);
        try {
            await invoke('download_map_region', { regionId });
        } catch (e) {
            setError(`Download failed: ${e}`);
            setActiveDownload(null);
        }
    };

    const handleDelete = async (regionId: string) => {
        if (!confirm('Delete this map pack? You can re-download it later.')) return;

        try {
            await invoke('delete_map_region', { regionId });
            await loadRegions();
        } catch (e) {
            setError(`Delete failed: ${e}`);
        }
    };

    const handleAddRegion = async () => {
        if (!selectedRegionId) return;
        try {
            await invoke('add_region', { regionId: selectedRegionId });
            await loadRegions();
            setSelectedRegionId('');
        } catch (error) {
            console.error('Failed to add region:', error);
            setError('Failed to add region');
        }
    };

    if (!isOpen) return null;

    return (
        <div className="modal-overlay" onClick={onClose}>
            <div className="modal-content" onClick={e => e.stopPropagation()}>
                <div className="modal-header">
                    <h2>🗺️ Map Packs</h2>
                    <button className="close-button" onClick={onClose}>×</button>
                </div>

                <p className="modal-description">
                    Download map data for offline use. Includes POI databases and routing data.
                </p>

                {error && <div className="error-message">{error}</div>}

                {loading ? (
                    <div className="loading">Loading regions...</div>
                ) : (
                    <div className="region-list">
                        {/* Add Region Section */}
                        <div className="add-region-section" style={{ padding: '16px', borderBottom: '1px solid rgba(255,255,255,0.1)', marginBottom: '16px' }}>
                            <div style={{ display: 'flex', gap: '10px' }}>
                                <select
                                    value={selectedRegionId}
                                    onChange={(e) => setSelectedRegionId(e.target.value)}
                                    style={{ flex: 1, padding: '8px', borderRadius: '4px', background: 'rgba(0,0,0,0.3)', color: 'white', border: '1px solid rgba(255,255,255,0.2)' }}
                                >
                                    <option value="">Select a region to add...</option>
                                    {availableRegions
                                        .filter(ar => !regions.some(r => r.id === ar.id)) // Filter out already added
                                        .sort((a, b) => a.name.localeCompare(b.name))
                                        .map(region => (
                                            <option key={region.id} value={region.id}>
                                                {region.name} ({region.size_mb} MB)
                                            </option>
                                        ))
                                    }
                                </select>
                                <button
                                    onClick={handleAddRegion}
                                    disabled={!selectedRegionId}
                                    className="download-button"
                                    style={{ padding: '8px 16px' }}
                                >
                                    Add
                                </button>
                            </div>
                        </div>

                        {regions.map(region => (
                            <div key={region.id} className={`region-item ${region.downloaded ? 'downloaded' : ''}`}>
                                <div className="region-info">
                                    <div className="region-name">
                                        {region.downloaded && <span className="check">✓</span>}
                                        {region.name}
                                    </div>
                                    <div className="region-meta">
                                        {region.size_mb} MB • {region.poi_count.toLocaleString()} POIs
                                        {region.last_updated && (
                                            <span className="updated"> • Updated {new Date(region.last_updated).toLocaleDateString()}</span>
                                        )}
                                    </div>
                                </div>

                                <div className="region-actions">
                                    {activeDownload === region.id ? (
                                        <div className="download-progress">
                                            <div
                                                className="progress-bar"
                                                style={{ width: `${downloadProgress?.progress_percent || 0}%` }}
                                            />
                                            <div className="progress-details">
                                                <span className="progress-text">
                                                    {downloadProgress?.status || 'Downloading...'}
                                                </span>
                                                <span className="progress-stats">
                                                    {downloadProgress && (
                                                        <>
                                                            {formatBytes(downloadProgress.bytes_downloaded)} / {formatBytes(downloadProgress.total_bytes)}
                                                            {' '}({Math.round(downloadProgress.progress_percent)}%)
                                                        </>
                                                    )}
                                                </span>
                                            </div>
                                        </div>
                                    ) : region.downloaded ? (
                                        <button
                                            className="delete-button"
                                            onClick={() => handleDelete(region.id)}
                                        >
                                            🗑️ Delete
                                        </button>
                                    ) : (
                                        <button
                                            className="download-button"
                                            onClick={() => handleDownload(region.id)}
                                            disabled={activeDownload !== null}
                                        >
                                            ⬇️ Download
                                        </button>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                )}

                <div className="modal-footer">
                    <span className="footer-info">
                        {regions.filter(r => r.downloaded).length} of {regions.length} packs downloaded
                    </span>
                </div>
            </div>
        </div>
    );
}
