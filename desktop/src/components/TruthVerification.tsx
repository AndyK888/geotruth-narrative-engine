import { useRef, useState, useEffect } from 'react';
import { MediaPlayer, MediaProvider } from '@vidstack/react';
import { DefaultVideoLayout, defaultLayoutIcons } from '@vidstack/react/player/layouts/default';
import 'vidstack/player/styles/default/theme.css';
import 'vidstack/player/styles/default/layouts/video.css';
import { convertFileSrc } from '@tauri-apps/api/core';
import { MapPin, MessageSquare, Clock, ArrowRight, Lock } from 'lucide-react';

interface VerificationEvent {
    id: string;
    timestamp: number; // seconds
    type: 'gps' | 'narration';
    content: string;
    verified: boolean;
}

interface TruthVerificationProps {
    videoPath: string;
    events: VerificationEvent[];
    onVerifyEvent?: (id: string, verified: boolean) => void;
    onAdjustTimestamp?: (id: string, newTimestamp: number) => void;
}

export function TruthVerification({ videoPath, events, onVerifyEvent, onAdjustTimestamp: _onAdjustTimestamp }: TruthVerificationProps) {
    const playerRef = useRef<any>(null);
    const listRef = useRef<HTMLDivElement>(null);
    const [currentTime, setCurrentTime] = useState(0);
    const [activeEventId, setActiveEventId] = useState<string | null>(null);
    const [lockToFrame, setLockToFrame] = useState(false);

    // Sort events by timestamp
    const sortedEvents = [...events].sort((a, b) => a.timestamp - b.timestamp);

    useEffect(() => {
        // Find active event (closest past event)
        // Or event that encompasses current time if we had duration.
        // For simplicity, let's highlight the event happening *now* or strictly after?
        // Usually "active" means the one we just passed.

        let current = null;
        for (let i = 0; i < sortedEvents.length; i++) {
            if (sortedEvents[i].timestamp <= currentTime) {
                current = sortedEvents[i].id;
            } else {
                break;
            }
        }
        setActiveEventId(current);

        // Auto-scroll logic could go here
        if (current && listRef.current) {
            const el = document.getElementById(`event-${current}`);
            if (el) {
                el.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
        }

    }, [currentTime, sortedEvents]);

    const handleJumpToEvent = (timestamp: number) => {
        if (playerRef.current) {
            playerRef.current.currentTime = timestamp;
        }
    };

    const handleTimeUpdate = (e: any) => {
        setCurrentTime(e.currentTime);
    };

    return (
        <div className="flex flex-col h-full bg-[var(--color-bg-primary)] text-[var(--color-text-primary)] overflow-hidden">
            {/* Top: Video Preview */}
            <div className="h-1/2 bg-black relative border-b border-[var(--color-border)] shadow-2xl z-10">
                <MediaPlayer
                    ref={playerRef}
                    src={convertFileSrc(videoPath)}
                    className="w-full h-full"
                    onTimeUpdate={handleTimeUpdate}
                >
                    <MediaProvider />
                    <DefaultVideoLayout icons={defaultLayoutIcons} />
                </MediaPlayer>

                {lockToFrame && (
                    <div className="absolute top-4 right-4 bg-red-600/90 text-white px-3 py-1 rounded-full text-[10px] font-bold flex items-center gap-1 animate-pulse shadow-lg backdrop-blur-md">
                        <Lock size={10} /> LOCKED TO FRAME
                    </div>
                )}
            </div>

            {/* Bottom: Event Stream */}
            <div className="h-1/2 flex flex-col bg-[var(--color-bg-secondary)] relative">
                {/* Header */}
                <div className="p-3 border-b border-[var(--color-border)] flex justify-between items-center bg-[var(--color-bg-tertiary)]/50 backdrop-blur-sm z-10">
                    <h3 className="font-bold text-xs text-[var(--color-text-secondary)] uppercase tracking-wider pl-2 flex items-center gap-2">
                        <div className="w-1.5 h-1.5 rounded-full bg-[var(--color-accent-primary)]"></div>
                        Timeline Events
                    </h3>
                    <div className="flex gap-2">
                        <button
                            className={`text-[10px] font-medium px-3 py-1.5 rounded-md border transition-all ${lockToFrame
                                ? 'bg-[var(--color-accent-primary)] border-[var(--color-accent-primary)] text-white shadow-lg shadow-blue-900/20'
                                : 'border-[var(--color-border)] text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-tertiary)] hover:text-[var(--color-text-primary)]'
                                }`}
                            onClick={() => setLockToFrame(!lockToFrame)}
                            title="Force narration to start at exact frame"
                        >
                            {lockToFrame ? 'Sync Locked' : 'Sync Unlocked'}
                        </button>
                    </div>
                </div>

                <div
                    ref={listRef}
                    className="flex-1 overflow-y-auto p-4 space-y-3 relative custom-scrollbar"
                >
                    {/* Sync Line Indicator (Visual Only) */}
                    <div className="absolute left-[88px] top-4 bottom-4 w-px bg-[var(--color-border)] pointer-events-none opacity-50" />

                    {sortedEvents.map((event) => {
                        const isActive = event.id === activeEventId;

                        return (
                            <div
                                id={`event-${event.id}`}
                                key={event.id}
                                className={`
                                    relative flex gap-4 p-3 rounded-xl border transition-all duration-300 group cursor-pointer
                                    ${isActive
                                        ? 'bg-[var(--color-bg-tertiary)] border-[var(--color-accent-primary)] shadow-lg shadow-black/20 scale-[1.01]'
                                        : 'bg-[var(--color-bg-tertiary)]/30 border-[var(--color-border)] hover:bg-[var(--color-bg-tertiary)] hover:border-[var(--color-border-hover)]'}
                                `}
                                onClick={() => handleJumpToEvent(event.timestamp)}
                            >
                                {/* Timestamp Column */}
                                <div className="w-16 flex flex-col items-center justify-center border-r border-[var(--color-border)] pr-4 relative">
                                    <span className={`font-mono text-sm tracking-tight ${isActive ? 'text-[var(--color-accent-primary)] font-bold' : 'text-[var(--color-text-muted)]'}`}>
                                        {new Date(event.timestamp * 1000).toISOString().substr(14, 5)}
                                    </span>
                                    <Clock size={12} className={`mt-1.5 ${isActive ? 'text-[var(--color-accent-primary)]' : 'text-[var(--color-text-muted)] opacity-50'}`} />

                                    {/* Timeline dot */}
                                    <div className={`absolute -right-[5px] top-1/2 -translate-y-1/2 w-2.5 h-2.5 rounded-full border-2 border-[var(--color-bg-secondary)] ${isActive ? 'bg-[var(--color-accent-primary)]' : 'bg-[var(--color-border)]'}`} />
                                </div>

                                {/* Content Column */}
                                <div className="flex-1 min-w-0">
                                    <div className="flex items-center gap-2 mb-1.5">
                                        {event.type === 'gps' ? (
                                            <span className="text-[9px] font-bold uppercase tracking-wider bg-indigo-500/10 text-indigo-400 border border-indigo-500/20 px-1.5 py-0.5 rounded flex items-center gap-1">
                                                <MapPin size={8} /> Location
                                            </span>
                                        ) : (
                                            <span className="text-[9px] font-bold uppercase tracking-wider bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-1.5 py-0.5 rounded flex items-center gap-1">
                                                <MessageSquare size={8} /> Narration
                                            </span>
                                        )}
                                        {isActive && <span className="text-[9px] text-[var(--color-accent-primary)] animate-pulse font-bold flex items-center gap-1">● ON AIR</span>}
                                    </div>

                                    <p className={`text-sm leading-relaxed ${isActive ? 'text-[var(--color-text-primary)]' : 'text-[var(--color-text-secondary)]'}`}>
                                        {event.content}
                                    </p>
                                </div>

                                {/* Action Buttons (Show on hover or active) */}
                                <div className={`flex flex-col justify-center gap-2 ${isActive ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'} transition-opacity p-1`}>
                                    <button
                                        onClick={(e) => { e.stopPropagation(); onVerifyEvent?.(event.id, !event.verified); }}
                                        className={`p-2 rounded-lg transition-all ${event.verified
                                            ? 'bg-green-500/20 text-green-400 hover:bg-green-500/30'
                                            : 'bg-[var(--color-bg-primary)] text-[var(--color-text-muted)] hover:bg-[var(--color-bg-glass-hover)] hover:text-[var(--color-text-primary)]'
                                            }`}
                                        title={event.verified ? "Verified" : "Mark as Verified"}
                                    >
                                        <ArrowRight size={14} />
                                    </button>
                                </div>
                            </div>
                        );
                    })}
                </div>
            </div>
        </div>
    );
}
