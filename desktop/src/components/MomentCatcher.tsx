import { useRef, useState, useEffect } from 'react';
import { MediaPlayer, MediaProvider, MediaPlayerInstance } from '@vidstack/react';
import { DefaultVideoLayout, defaultLayoutIcons } from '@vidstack/react/player/layouts/default';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import 'vidstack/player/styles/default/theme.css';
import 'vidstack/player/styles/default/layouts/video.css';
import { Camera, Sparkles, Terminal } from 'lucide-react';

interface MomentCatcherProps {
    videoPath: string;
    onMomentCaptured?: (data: { timestamp: number; image: string; description?: string }) => void;
}

export function MomentCatcher({ videoPath, onMomentCaptured }: MomentCatcherProps) {
    const playerRef = useRef<MediaPlayerInstance>(null);
    const [analyzing, setAnalyzing] = useState(false);
    const [lastCaptureTime, setLastCaptureTime] = useState<number | null>(null);
    const [debugLogs, setDebugLogs] = useState<string[]>([]);

    const log = (msg: string) => {
        setDebugLogs(prev => [`[${new Date().toLocaleTimeString()}] ${msg}`, ...prev].slice(0, 5));
        console.log(`[VideoPlayer] ${msg}`);
    };

    useEffect(() => {
        log(`Initialized with path: ${videoPath}`);
        log(`Converted src: ${convertFileSrc(videoPath)}`);
    }, [videoPath]);

    const handleCapture = async () => {
        if (!playerRef.current) return;

        const time = playerRef.current.currentTime;
        const timestampMs = Math.floor(time * 1000);

        setAnalyzing(true);
        try {
            // 1. Capture Frame (Rust)
            const base64Image = await invoke<string>('capture_frame', {
                videoPath,
                timestampMs
            });

            // 2. Here we would fetch GPS location if available
            // const location = await invoke('get_gps_at_time', { videoPath, timestampMs });

            // Simulate AI delay
            setTimeout(() => {
                const mockDescription = "Looking out over Big Sur coastline from Highway 1. (Simulated AI)";

                // Notify parent
                onMomentCaptured?.({
                    timestamp: time,
                    image: base64Image,
                    description: mockDescription
                });

                setLastCaptureTime(time);
                setAnalyzing(false);
            }, 800);

        } catch (err) {
            console.error("Failed to capture moment:", err);
            setAnalyzing(false);
            // Could add toast here
        }
    };

    // Keyboard shortcut 'c'
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key.toLowerCase() === 'c') {
                handleCapture();
            }
        };
        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [videoPath]);

    return (
        <div className="w-full h-full relative bg-black group flex flex-col justify-center items-center">
            <MediaPlayer
                ref={playerRef}
                src={convertFileSrc(videoPath)}
                aspectRatio="16/9"
                className="w-full h-full object-contain"
                onLoadStart={() => log('Loading started...')}
                onLoadedMetadata={() => log('Metadata loaded')}
                onCanPlay={() => log('Can play')}
                onWaiting={() => log('Buffering/Waiting...')}
                onError={(detail) => log(`Error: ${JSON.stringify(detail)}`)}
                onSuspend={() => log('Suspended (download complete or stopped)')}
                onStalled={() => log('Stalled (data fetch slow)')}
            >
                <MediaProvider />
                <DefaultVideoLayout icons={defaultLayoutIcons} />
            </MediaPlayer>

            {/* Debug Console Overlay */}
            <div className="absolute top-4 left-4 z-50 bg-black/80 text-green-400 p-4 rounded-lg font-mono text-xs max-w-md pointer-events-none border border-green-900 shadow-xl backdrop-blur-sm">
                <div className="flex items-center gap-2 border-b border-green-900 pb-2 mb-2">
                    <Terminal className="w-3 h-3" />
                    <span className="font-bold">Player Debug Console</span>
                </div>
                <div className="space-y-1">
                    {debugLogs.length === 0 && <span className="opacity-50">Waiting for events...</span>}
                    {debugLogs.map((log, i) => (
                        <div key={i} className="break-all">{log}</div>
                    ))}
                </div>
            </div>

            {/* Previous gradient overlay... */}

            {/* Gradient Overlay for controls visibility */}
            <div className="absolute inset-x-0 bottom-0 h-32 bg-gradient-to-t from-black/80 to-transparent pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-300" />

            {/* Floating Action Button */}
            <div className="absolute bottom-8 right-8 z-10">
                <button
                    onClick={handleCapture}
                    disabled={analyzing}
                    className={`
                        flex items-center gap-2 px-6 py-3 rounded-full font-bold shadow-2xl transition-all transform
                        ${analyzing
                            ? 'bg-slate-800 text-slate-400 cursor-wait scale-95'
                            : 'bg-white text-black hover:scale-105 hover:shadow-glow'}
                    `}
                >
                    {analyzing ? (
                        <>
                            <Sparkles size={20} className="animate-spin" />
                            Analyzing...
                        </>
                    ) : (
                        <>
                            <Camera size={20} />
                            Analyze Moment <span className="text-xs opacity-50 font-mono ml-1">(C)</span>
                        </>
                    )}
                </button>
            </div>

            {/* Flash Effect on Capture */}
            {lastCaptureTime && (
                <div
                    key={lastCaptureTime}
                    className="absolute inset-0 bg-white pointer-events-none animate-flash"
                />
            )}
        </div>
    );
}

// Add simple flash animation style
const style = document.createElement('style');
style.textContent = `
@keyframes flash {
    0% { opacity: 0.5; }
    100% { opacity: 0; }
}
.animate-flash {
    animation: flash 0.3s ease-out forwards;
}
`;
document.head.appendChild(style);
