import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

function App() {
    const [appVersion, setAppVersion] = useState<string>('');
    const [connectionStatus, setConnectionStatus] = useState<'online' | 'offline' | 'checking'>(
        'checking'
    );

    useEffect(() => {
        // Get app version from Rust backend
        invoke<string>('get_version')
            .then((version) => setAppVersion(version))
            .catch(console.error);

        // Check API connection status
        checkConnection();
    }, []);

    const checkConnection = async () => {
        setConnectionStatus('checking');
        try {
            const isOnline = await invoke<boolean>('check_api_connection');
            setConnectionStatus(isOnline ? 'online' : 'offline');
        } catch {
            setConnectionStatus('offline');
        }
    };

    return (
        <div className="app">
            <header className="app-header">
                <div className="logo">
                    <svg viewBox="0 0 24 24" className="logo-icon">
                        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
                    </svg>
                    <h1>GeoTruth</h1>
                </div>
                <div className="status-bar">
                    <span
                        className={`status-indicator ${connectionStatus}`}
                        title={`Mode: ${connectionStatus === 'online' ? 'Online' : 'Offline'}`}
                    >
                        {connectionStatus === 'checking' ? '⏳' : connectionStatus === 'online' ? '🌐' : '📴'}
                    </span>
                    <span className="version">v{appVersion || '...'}</span>
                </div>
            </header>

            <main className="app-main">
                <section className="welcome-section">
                    <h2>Welcome to GeoTruth Narrative Engine</h2>
                    <p className="subtitle">Turn raw travel footage into fact-checked, AI-narrated stories</p>

                    <div className="feature-cards">
                        <div className="feature-card">
                            <div className="feature-icon">🎥</div>
                            <h3>Import Video</h3>
                            <p>Drag & drop your travel footage with GPS data</p>
                        </div>
                        <div className="feature-card">
                            <div className="feature-icon">🔍</div>
                            <h3>Verify Facts</h3>
                            <p>Automatically validate locations using geospatial databases</p>
                        </div>
                        <div className="feature-card">
                            <div className="feature-icon">🤖</div>
                            <h3>AI Narration</h3>
                            <p>Generate fact-checked scripts for your videos</p>
                        </div>
                    </div>

                    <div className="action-buttons">
                        <button className="primary-button" disabled>
                            <span className="button-icon">📁</span>
                            Import Project
                            <span className="coming-soon">Coming Soon</span>
                        </button>
                        <button className="secondary-button" disabled>
                            <span className="button-icon">➕</span>
                            New Project
                            <span className="coming-soon">Coming Soon</span>
                        </button>
                    </div>
                </section>

                <section className="status-section">
                    <h3>System Status</h3>
                    <div className="status-grid">
                        <div className="status-item">
                            <span className="status-label">Mode</span>
                            <span className={`status-value ${connectionStatus}`}>
                                {connectionStatus === 'checking'
                                    ? 'Checking...'
                                    : connectionStatus === 'online'
                                        ? 'Online (Docker API)'
                                        : 'Offline (Local)'}
                            </span>
                        </div>
                        <div className="status-item">
                            <span className="status-label">Processing</span>
                            <span className="status-value ready">Ready</span>
                        </div>
                        <div className="status-item">
                            <span className="status-label">Map Packs</span>
                            <span className="status-value">Not Downloaded</span>
                        </div>
                    </div>
                    <button className="retry-button" onClick={checkConnection}>
                        🔄 Refresh Connection
                    </button>
                </section>
            </main>

            <footer className="app-footer">
                <p>
                    <strong>Privacy First:</strong> All video processing happens locally on your machine.
                </p>
            </footer>
        </div>
    );
}

export default App;
