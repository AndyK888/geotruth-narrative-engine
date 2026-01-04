import { useEffect, useState } from 'react';
import { Timeline, TimelineEffect, type TimelineRow as TimelineRowType } from '@xzdarcy/react-timeline-editor';


interface TimelineEvent {
    id: string;
    action: string;
    startTime: number;
    endTime: number;
    text?: string;
    // ... other props
}

interface TruthTimelineProProps {
    duration: number; // total video duration in seconds
    events: TimelineEvent[];
    narrationTracks: TimelineEvent[];
    onEventChange?: (updatedEvent: TimelineEvent) => void;
    currentTime: number;
    onSeek?: (time: number) => void;
}

export function TruthTimelinePro({ duration: _duration, events, narrationTracks, onEventChange: _onEventChange, currentTime, onSeek }: TruthTimelineProProps) {
    // Map inputs to timeline editor data format
    const [editorData, setEditorData] = useState<TimelineRowType[]>([]);

    useEffect(() => {
        // Construct rows
        const gpsRow: TimelineRowType = {
            id: 'gps_track',
            actions: events.map(e => ({
                id: e.id,
                start: e.startTime,
                end: e.endTime,
                effectId: 'gps',
                data: { title: e.text || 'Location POI' }
            }))
        };

        const narrationRow: TimelineRowType = {
            id: 'narration_track',
            actions: narrationTracks.map(e => ({
                id: e.id,
                start: e.startTime,
                end: e.endTime,
                effectId: 'narration',
                data: { title: e.text || 'Narration' }
            }))
        };

        setEditorData([gpsRow, narrationRow]);
    }, [events, narrationTracks]);

    const mockEffect: Record<string, TimelineEffect> = {
        gps: {
            id: 'gps',
            name: 'Location Match',
            source: {
                start: () => { },
                enter: () => { },
                leave: () => { },
                stop: () => { },
            }
        },
        narration: {
            id: 'narration',
            name: 'AI Voice',
            source: {
                start: () => { },
                enter: () => { },
                leave: () => { },
                stop: () => { },
            }
        },
    };

    return (
        <div className="w-full bg-slate-900 border border-slate-700 rounded-lg overflow-hidden h-64 flex flex-col">
            <div className="p-2 border-b border-slate-700 flex justify-between items-center text-xs text-slate-400">
                <span>Timeline Editor</span>
                <span>{currentTime.toFixed(2)}s</span>
            </div>

            <div className="flex-1 overflow-auto custom-timeline-wrapper">
                <Timeline
                    scale={50} // pixels per second
                    scaleWidth={160}
                    startLeft={20}
                    autoScroll={true}
                    editorData={editorData}
                    effects={mockEffect}

                    onChange={(data: TimelineRowType[]) => {
                        console.log('Timeline changed', data);
                        setEditorData(data);
                        // Extract changed item and call onEventChange if real implementation
                    }}

                    getActionRender={(action, _row) => {
                        const isGps = action.effectId === 'gps';
                        return (
                            <div className={`h-full w-full rounded flex items-center px-2 text-xs truncate overflow-hidden cursor-move ${isGps ? 'bg-blue-600 border border-blue-400' : 'bg-emerald-600 border border-emerald-400'}`}>
                                {(action as any).data.title}
                            </div>
                        );
                    }}

                    onCursorDrag={(time) => {
                        onSeek?.(time);
                    }}

                // Controlled time if supported or just cursor
                />
            </div>

            <style>{`
                .custom-timeline-wrapper .timeline-editor {
                     background: #0f172a; /* slate-900 */
                }
            `}</style>
        </div>
    );
}
