import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './MapPacksModal.css';

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
            const data = await invoke<RegionInfo[]>('get_map_regions');
            setRegions(data);

            const downloaded = data.filter(r => r.downloaded).length;
            onStatusChange(downloaded, data.length);
        } catch (e) {
            setError(`Failed to load regions: ${e}`);
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
                                            <span className="progress-text">
                                                {downloadProgress?.status || 'Downloading...'}
                                            </span>
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
