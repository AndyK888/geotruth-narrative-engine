import { useRef, useState, useEffect, useMemo } from 'react';
import { MediaPlayer, MediaProvider, MediaPlayerInstance, Track } from '@vidstack/react';
import { VideoLayout } from './player/VideoLayout';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import 'vidstack/player/styles/default/theme.css';

interface MomentCatcherProps {
    videoPath: string;
    onMomentCaptured?: (data: { timestamp: number; image: string; description?: string }) => void;
    onAutoCaptured?: (moments: Array<{ timestamp: number; image: string; description?: string }>) => void;
    moments?: Array<{ timestamp: number; description?: string }>; // For timeline markers
}

export function MomentCatcher({ videoPath, onMomentCaptured, onAutoCaptured, moments = [] }: MomentCatcherProps) {
    const playerRef = useRef<MediaPlayerInstance>(null);
    const [analyzing, setAnalyzing] = useState(false);
    const [lastCaptureTime, setLastCaptureTime] = useState<number | null>(null);

    // Generate VTT for Chapters/Markers
    const chaptersSrc = useMemo(() => {
        if (moments.length === 0) return undefined;

        let vttContent = "WEBVTT\n\n";
        moments.forEach((m, i) => {
            // Create a chapter that lasts 1 second or until next moment
            const start = m.timestamp;
            const end = (i < moments.length - 1) ? moments[i + 1].timestamp : start + 5;

            const formatTime = (s: number) => new Date(s * 1000).toISOString().substr(11, 8) + ".000";

            vttContent += `${formatTime(start)} --> ${formatTime(end)}\n`;
            vttContent += `${m.description || 'Moment ' + (i + 1)}\n\n`;
        });

        const blob = new Blob([vttContent], { type: 'text/vtt' });
        return URL.createObjectURL(blob);
    }, [moments]);

    // Cleanup Blob URL
    useEffect(() => {
        return () => {
            if (chaptersSrc) URL.revokeObjectURL(chaptersSrc);
        };
    }, [chaptersSrc]);

    // Debug logging (console only)
    const log = (msg: string) => console.log(`[VideoPlayer] ${msg}`);

    useEffect(() => {
        log(`Initialized with path: ${videoPath}`);
        log(`Converted src: ${convertFileSrc(videoPath)}`);
    }, [videoPath]);

    const handleCapture = async () => {
        if (!playerRef.current) return;

        const time = playerRef.current.currentTime;
        const timestampMs = Math.floor(time * 1000);

        setAnalyzing(true);
        log(`Capturing frame at ${time.toFixed(2)}s...`);
        try {
            // 1. Capture Frame (Rust)
            const base64Image = await invoke<string>('capture_frame', {
                videoPath,
                timestampMs
            });

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
                log('Moment captured successfully');
            }, 800);

        } catch (err) {
            console.error("Failed to capture moment:", err);
            log(`Capture failed: ${err}`);
            setAnalyzing(false);
        }
    };

    const handleAutoAnalyze = async () => {
        setAnalyzing(true);
        log('Starting auto-analysis (scanning video)...');
        try {
            const scannedMoments = await invoke<Array<{ timestamp: number; image_path: string }>>('auto_scan_moments', {
                videoPath
            });

            log(`Scanned ${scannedMoments.length} moments. converting...`);

            // Convert paths to asset URLs
            const moments = scannedMoments.map(m => ({
                timestamp: m.timestamp,
                image: convertFileSrc(m.image_path), // Use asset protocol
                description: `Auto-Scanned Scene at ${m.timestamp}s`
            }));

            onAutoCaptured?.(moments);
            setAnalyzing(false);
            log('Auto-analysis complete.');
        } catch (err) {
            console.error("Auto-scan failed:", err);
            log(`Auto-scan failed: ${err}`);
            setAnalyzing(false);
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
        <div className="w-full h-full relative bg-black group flex flex-col justify-center items-center overflow-hidden rounded-xl">
            <MediaPlayer
                ref={playerRef}
                src={convertFileSrc(videoPath)}
                aspectRatio="16/9"
                className="w-full h-full object-contain"
                onLoadStart={() => log('Loading started...')}
                onLoadedMetadata={() => log('Metadata loaded')}
                onCanPlay={() => log('Can play')}
            >
                <MediaProvider>
                    {chaptersSrc && <Track src={chaptersSrc} kind="chapters" label="Moments" default />}
                </MediaProvider>

                <VideoLayout
                    onCapture={handleCapture}
                    onDetect={handleAutoAnalyze}
                    isAnalyzing={analyzing}
                />
            </MediaPlayer>

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
