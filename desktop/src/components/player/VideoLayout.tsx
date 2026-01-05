import {
    CaptionButton,
    FullscreenButton,
    MuteButton,
    PIPButton,
    PlayButton,
    TimeSlider,
    Time,
    VolumeSlider,
    useMediaState,
} from '@vidstack/react';
import {
    Maximize,
    Volume2,
    Play,
    Pause,
    PictureInPicture,
    Subtitles,
    Camera,
    Search // For 'Detect'
} from 'lucide-react';

interface VideoLayoutProps {
    onCapture?: () => void;
    onDetect?: () => void;
    isAnalyzing?: boolean;
}

export function VideoLayout({ onCapture, onDetect, isAnalyzing }: VideoLayoutProps) {
    const isPaused = useMediaState('paused');
    const duration = useMediaState('duration');

    return (
        <div className="pointer-events-none absolute inset-0 flex flex-col justify-end p-4 bg-gradient-to-t from-black/90 via-black/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300">

            {/* Time Slider */}
            <div className="pointer-events-auto w-full mb-4 group/slider">
                <TimeSlider.Root className="relative flex items-center select-none touch-none w-full h-4 group-hover/slider:h-6 transition-all">
                    <TimeSlider.Chapters className="relative flex w-full h-1 bg-white/20 rounded-sm">
                        {(cues) => cues.map((cue) => {
                            // Safety check: avoid division by zero
                            const safeDuration = duration && duration > 0 ? duration : 1;
                            const widthPercent = ((cue.endTime - cue.startTime) / safeDuration) * 100;
                            return (
                                <div
                                    key={cue.startTime}
                                    className="h-full bg-white/40 last:rounded-r-sm first:rounded-l-sm relative"
                                    style={{ width: `${widthPercent}% ` }}
                                />
                            );
                        })}
                    </TimeSlider.Chapters>

                    <TimeSlider.Track className="absolute inset-0 w-full h-1 group-hover/slider:h-2 bg-white/20 rounded-sm overflow-hidden content-center self-center my-auto transition-all">
                        <TimeSlider.TrackFill className="bg-[var(--color-accent-primary)] h-full absolute top-0 left-0" />
                        <TimeSlider.Progress className="bg-white/50 h-full absolute top-0 left-0" />
                    </TimeSlider.Track>

                    <TimeSlider.Thumb className="absolute top-1/2 -translate-y-1/2 w-3 h-3 group-hover/slider:w-4 group-hover/slider:h-4 bg-white rounded-full opacity-0 group-hover/slider:opacity-100 transition-all shadow-lg ring-2 ring-[var(--color-accent-primary)]/50" />

                    {/* Time Preview */}
                    <TimeSlider.Preview className="flex flex-col items-center opacity-0 data-[visible]:opacity-100 transition-opacity">
                        <div className="bg-black/90 text-white text-xs px-2 py-1 rounded mb-2 font-mono border border-white/10">
                            <TimeSlider.Value />
                        </div>
                    </TimeSlider.Preview>
                </TimeSlider.Root>
            </div>

            {/* Control Bar */}
            <div className="pointer-events-auto flex items-center justify-between w-full gap-4">

                {/* Left Group */}
                <div className="flex items-center gap-4">
                    <PlayButton className="hover:text-[var(--color-accent-primary)] transition-colors text-white">
                        {isPaused ? <Play fill="currentColor" size={24} /> : <Pause fill="currentColor" size={24} />}
                    </PlayButton>

                    <div className="group/volume flex items-center gap-2">
                        <MuteButton className="hover:text-[var(--color-accent-primary)] transition-colors text-white">
                            <Volume2 size={20} />
                        </MuteButton>
                        <VolumeSlider.Root className="w-0 group-hover/volume:w-20 transition-all h-1 bg-white/20 rounded-full relative ml-1 p-0.5">
                            <VolumeSlider.Track className="bg-white/20 h-full w-full rounded-full absolute top-0 left-0">
                                <VolumeSlider.TrackFill className="bg-white h-full absolute top-0 left-0 rounded-full" />
                            </VolumeSlider.Track>
                            <VolumeSlider.Thumb className="w-3 h-3 bg-white rounded-full absolute top-1/2 -translate-y-1/2 shadow opacity-0 group-hover/volume:opacity-100" />
                        </VolumeSlider.Root>
                    </div>

                    <div className="text-xs font-mono text-white/80 select-none flex items-center gap-1">
                        <Time type="current" /> / <Time type="duration" />
                    </div>
                </div>

                {/* Center/Right Actions (Custom) */}
                <div className="flex items-center gap-3">
                    {onDetect && (
                        <button
                            onClick={onDetect}
                            disabled={isAnalyzing}
                            className={`
                                flex items-center gap-2 px-4 py-2 rounded-full font-semibold text-sm
                                bg-purple-600 text-white shadow-lg
                                hover:bg-purple-500 hover:scale-105 
                                disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100
                                transition-all duration-200
                                ${isAnalyzing ? 'animate-pulse' : ''}
                            `}
                            title="Detect Key Moments"
                        >
                            <Search size={16} className={isAnalyzing ? 'animate-spin' : ''} />
                            <span className="hidden sm:inline">Detect</span>
                        </button>
                    )}
                    {onCapture && (
                        <button
                            onClick={onCapture}
                            disabled={isAnalyzing}
                            className={`
                                flex items-center gap-2 px-4 py-2 rounded-full font-semibold text-sm
                                bg-white text-black shadow-lg
                                hover:scale-105 hover:shadow-xl
                                disabled:bg-slate-600 disabled:text-slate-400 disabled:cursor-not-allowed disabled:hover:scale-100
                                transition-all duration-200
                            `}
                            title="Capture Frame (C)"
                        >
                            <Camera size={16} />
                            <span className="hidden sm:inline">Capture</span>
                            <span className="text-[10px] opacity-50 font-mono hidden sm:inline">(C)</span>
                        </button>
                    )}
                </div>

                {/* Right Group */}
                <div className="flex items-center gap-4">
                    <CaptionButton className="hover:text-[var(--color-accent-primary)] transition-colors text-white">
                        <Subtitles size={20} />
                    </CaptionButton>
                    <PIPButton className="hover:text-[var(--color-accent-primary)] transition-colors text-white">
                        <PictureInPicture size={20} />
                    </PIPButton>
                    <FullscreenButton className="hover:text-[var(--color-accent-primary)] transition-colors text-white">
                        <Maximize size={20} />
                    </FullscreenButton>
                </div>

            </div>
        </div>
    );
}
