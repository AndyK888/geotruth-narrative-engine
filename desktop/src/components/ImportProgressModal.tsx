import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Loader2, CheckCircle2, Upload, Database, MapPin, FileVideo } from 'lucide-react';

interface ImportProgress {
    stage: string;
    progress: number;
    message: string;
}

interface ImportProgressModalProps {
    isOpen: boolean;
    onComplete?: () => void;
}

export function ImportProgressModal({ isOpen, onComplete }: ImportProgressModalProps) {
    const [progress, setProgress] = useState<ImportProgress>({
        stage: 'start',
        progress: 0,
        message: 'Starting import...'
    });

    useEffect(() => {
        if (!isOpen) return;

        const unlisten = listen<ImportProgress>('import-progress', (event) => {
            setProgress(event.payload);

            if (event.payload.stage === 'complete') {
                // Delay before closing to show complete state
                setTimeout(() => onComplete?.(), 800);
            }
        });

        return () => {
            unlisten.then(fn => fn());
        };
    }, [isOpen, onComplete]);

    if (!isOpen) return null;

    const getStageIcon = (stage: string) => {
        switch (stage) {
            case 'start':
                return <Upload className="w-5 h-5" />;
            case 'metadata':
                return <FileVideo className="w-5 h-5" />;
            case 'gps':
                return <MapPin className="w-5 h-5" />;
            case 'database':
                return <Database className="w-5 h-5" />;
            case 'complete':
                return <CheckCircle2 className="w-5 h-5 text-green-400" />;
            default:
                return <Loader2 className="w-5 h-5 animate-spin" />;
        }
    };

    const isComplete = progress.stage === 'complete';

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm">
            <div className="bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded-2xl p-8 w-full max-w-md shadow-2xl">
                {/* Header */}
                <div className="flex items-center gap-4 mb-6">
                    <div className={`p-3 rounded-xl ${isComplete ? 'bg-green-500/20' : 'bg-[var(--color-accent-primary)]/20'}`}>
                        {isComplete ? (
                            <CheckCircle2 className="w-8 h-8 text-green-400" />
                        ) : (
                            <Loader2 className="w-8 h-8 text-[var(--color-accent-primary)] animate-spin" />
                        )}
                    </div>
                    <div>
                        <h2 className="text-xl font-bold text-white">
                            {isComplete ? 'Import Complete' : 'Importing Video'}
                        </h2>
                        <p className="text-sm text-[var(--color-text-secondary)]">
                            {progress.message}
                        </p>
                    </div>
                </div>

                {/* Progress Bar */}
                <div className="mb-6">
                    <div className="flex justify-between text-xs text-[var(--color-text-secondary)] mb-2">
                        <span>Progress</span>
                        <span className="font-mono">{progress.progress}%</span>
                    </div>
                    <div className="h-3 bg-[var(--color-bg-tertiary)] rounded-full overflow-hidden">
                        <div
                            className={`h-full transition-all duration-500 ease-out rounded-full ${isComplete
                                    ? 'bg-green-500'
                                    : 'bg-gradient-to-r from-[var(--color-accent-primary)] to-[var(--color-accent-secondary)]'
                                }`}
                            style={{ width: `${progress.progress}%` }}
                        />
                    </div>
                </div>

                {/* Stage Indicators */}
                <div className="grid grid-cols-5 gap-2">
                    {['start', 'metadata', 'gps', 'database', 'complete'].map((stage, i) => {
                        const stageProgress = ['start', 'metadata', 'gps', 'database', 'complete'].indexOf(progress.stage);
                        const thisStageIndex = i;
                        const isActive = thisStageIndex === stageProgress;
                        const isDone = thisStageIndex < stageProgress;

                        return (
                            <div
                                key={stage}
                                className={`flex flex-col items-center gap-1 p-2 rounded-lg transition-all ${isActive
                                        ? 'bg-[var(--color-accent-primary)]/20 text-[var(--color-accent-primary)]'
                                        : isDone
                                            ? 'text-green-400'
                                            : 'text-[var(--color-text-muted)]'
                                    }`}
                            >
                                {getStageIcon(stage)}
                                <span className="text-[10px] capitalize">{stage === 'gps' ? 'GPS' : stage}</span>
                            </div>
                        );
                    })}
                </div>
            </div>
        </div>
    );
}
