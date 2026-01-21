import { useState, useMemo, useCallback } from 'react';

interface TruthEvent {
  id: string;
  videoTimeSeconds: number;
  lat: number;
  lon: number;
  pois: {
    id: string;
    name: string;
    category: string;
    distance_m: number;
  }[];
  verified: boolean;
  verificationMode: 'online' | 'offline' | 'pending';
  confidence: number;
}

interface TruthTimelineProps {
  events: TruthEvent[];
  videoDuration: number;
  currentTime: number;
  onSeek: (time: number) => void;
  onEventSelect?: (event: TruthEvent) => void;
}

/**
 * Truth Timeline - Interactive timeline showing verified events
 */
export function TruthTimeline({
  events,
  videoDuration,
  currentTime,
  onSeek,
  onEventSelect,
}: TruthTimelineProps) {
  const [selectedEvent, setSelectedEvent] = useState<TruthEvent | null>(null);
  const [hoveredEvent, setHoveredEvent] = useState<TruthEvent | null>(null);

  // Calculate event positions
  const eventPositions = useMemo(() => {
    return events.map((event) => ({
      event,
      position: (event.videoTimeSeconds / videoDuration) * 100,
    }));
  }, [events, videoDuration]);

  // Current playhead position
  const playheadPosition = (currentTime / videoDuration) * 100;

  // Handle click on timeline
  const handleTimelineClick = useCallback(
    (e: React.MouseEvent<HTMLDivElement>) => {
      const rect = e.currentTarget.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const percentage = x / rect.width;
      const newTime = percentage * videoDuration;
      onSeek(Math.max(0, Math.min(newTime, videoDuration)));
    },
    [videoDuration, onSeek]
  );

  // Handle event click
  const handleEventClick = (event: TruthEvent, e: React.MouseEvent) => {
    e.stopPropagation();
    setSelectedEvent(event);
    onSeek(event.videoTimeSeconds);
    onEventSelect?.(event);
  };

  // Format time for display
  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  // Get color for verification status
  const getEventColor = (event: TruthEvent): string => {
    if (!event.verified) return 'var(--color-warning)';
    if (event.confidence >= 0.9) return 'var(--color-success)';
    if (event.confidence >= 0.6) return '#60a5fa';
    return 'var(--color-warning)';
  };

  return (
    <div className="truth-timeline">
      <div className="timeline-header">
        <h3>Truth Timeline</h3>
        <span className="event-count">{events.length} verified events</span>
      </div>

      <div className="timeline-container" onClick={handleTimelineClick}>
        {/* Timeline track */}
        <div className="timeline-track">
          {/* Progress fill */}
          <div className="timeline-progress" style={{ width: `${playheadPosition}%` }} />

          {/* Event markers */}
          {eventPositions.map(({ event, position }) => (
            <div
              key={event.id}
              className={`event-marker ${selectedEvent?.id === event.id ? 'selected' : ''}`}
              style={{
                left: `${position}%`,
                backgroundColor: getEventColor(event),
              }}
              onClick={(e) => handleEventClick(event, e)}
              onMouseEnter={() => setHoveredEvent(event)}
              onMouseLeave={() => setHoveredEvent(null)}
            >
              {event.pois.length > 0 && <span className="poi-count">{event.pois.length}</span>}
            </div>
          ))}

          {/* Playhead */}
          <div className="playhead" style={{ left: `${playheadPosition}%` }} />
        </div>

        {/* Time labels */}
        <div className="time-labels">
          <span>0:00</span>
          <span>{formatTime(videoDuration / 4)}</span>
          <span>{formatTime(videoDuration / 2)}</span>
          <span>{formatTime((videoDuration * 3) / 4)}</span>
          <span>{formatTime(videoDuration)}</span>
        </div>
      </div>

      {/* Hover tooltip */}
      {hoveredEvent && (
        <div className="event-tooltip">
          <div className="tooltip-time">{formatTime(hoveredEvent.videoTimeSeconds)}</div>
          <div className="tooltip-location">
            {hoveredEvent.lat.toFixed(4)}, {hoveredEvent.lon.toFixed(4)}
          </div>
          {hoveredEvent.pois.length > 0 && (
            <div className="tooltip-pois">
              <strong>Nearby:</strong>
              <ul>
                {hoveredEvent.pois.slice(0, 3).map((poi) => (
                  <li key={poi.id}>
                    {poi.name} ({Math.round(poi.distance_m)}m)
                  </li>
                ))}
              </ul>
            </div>
          )}
          <div className="tooltip-status">
            <span
              className={`status-badge ${hoveredEvent.verificationMode}`}
              style={{ backgroundColor: getEventColor(hoveredEvent) }}
            >
              {hoveredEvent.verified ? '✓ Verified' : '⏳ Pending'}
            </span>
            <span className="confidence">{Math.round(hoveredEvent.confidence * 100)}%</span>
          </div>
        </div>
      )}

      {/* Selected event details */}
      {selectedEvent && (
        <div className="event-details">
          <h4>Event Details</h4>
          <button className="close-btn" onClick={() => setSelectedEvent(null)}>
            ×
          </button>

          <div className="detail-grid">
            <div className="detail-item">
              <label>Time</label>
              <span>{formatTime(selectedEvent.videoTimeSeconds)}</span>
            </div>
            <div className="detail-item">
              <label>Location</label>
              <span>
                {selectedEvent.lat.toFixed(6)}, {selectedEvent.lon.toFixed(6)}
              </span>
            </div>
            <div className="detail-item">
              <label>Verification</label>
              <span>{selectedEvent.verificationMode}</span>
            </div>
            <div className="detail-item">
              <label>Confidence</label>
              <span>{Math.round(selectedEvent.confidence * 100)}%</span>
            </div>
          </div>

          {selectedEvent.pois.length > 0 && (
            <div className="poi-list">
              <h5>Points of Interest</h5>
              {selectedEvent.pois.map((poi) => (
                <div key={poi.id} className="poi-item">
                  <span className="poi-name">{poi.name}</span>
                  <span className="poi-category">{poi.category}</span>
                  <span className="poi-distance">{Math.round(poi.distance_m)}m</span>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      <style>{`
        .truth-timeline {
          background: var(--color-bg-glass);
          border: 1px solid var(--color-border);
          border-radius: var(--radius-lg);
          padding: var(--spacing-lg);
          margin: var(--spacing-md) 0;
        }

        .timeline-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: var(--spacing-md);
        }

        .timeline-header h3 {
          font-size: var(--font-size-lg);
          color: var(--color-text-primary);
          margin: 0;
        }

        .event-count {
          font-size: var(--font-size-sm);
          color: var(--color-text-secondary);
        }

        .timeline-container {
          position: relative;
          cursor: pointer;
          padding: var(--spacing-md) 0;
        }

        .timeline-track {
          position: relative;
          height: 8px;
          background: var(--color-bg-tertiary);
          border-radius: var(--radius-full);
          overflow: visible;
        }

        .timeline-progress {
          position: absolute;
          top: 0;
          left: 0;
          height: 100%;
          background: var(--color-accent-gradient);
          border-radius: var(--radius-full);
          transition: width 0.1s linear;
        }

        .event-marker {
          position: absolute;
          top: 50%;
          transform: translate(-50%, -50%);
          width: 14px;
          height: 14px;
          border-radius: 50%;
          cursor: pointer;
          transition: transform 0.15s ease, box-shadow 0.15s ease;
          z-index: 1;
        }

        .event-marker:hover,
        .event-marker.selected {
          transform: translate(-50%, -50%) scale(1.3);
          box-shadow: 0 0 12px currentColor;
        }

        .event-marker .poi-count {
          position: absolute;
          top: -8px;
          right: -8px;
          background: var(--color-accent-primary);
          color: white;
          font-size: 10px;
          width: 16px;
          height: 16px;
          border-radius: 50%;
          display: flex;
          align-items: center;
          justify-content: center;
          font-weight: bold;
        }

        .playhead {
          position: absolute;
          top: -4px;
          width: 2px;
          height: 16px;
          background: white;
          transform: translateX(-50%);
          border-radius: 2px;
          z-index: 2;
        }

        .time-labels {
          display: flex;
          justify-content: space-between;
          margin-top: var(--spacing-sm);
          font-size: var(--font-size-xs);
          color: var(--color-text-muted);
        }

        .event-tooltip {
          position: absolute;
          bottom: 100%;
          left: 50%;
          transform: translateX(-50%);
          background: var(--color-bg-secondary);
          border: 1px solid var(--color-border);
          border-radius: var(--radius-md);
          padding: var(--spacing-sm);
          min-width: 200px;
          margin-bottom: var(--spacing-sm);
          z-index: 10;
        }

        .event-details {
          margin-top: var(--spacing-lg);
          padding: var(--spacing-md);
          background: var(--color-bg-secondary);
          border-radius: var(--radius-md);
          position: relative;
        }

        .event-details h4 {
          margin: 0 0 var(--spacing-md);
          color: var(--color-text-primary);
        }

        .close-btn {
          position: absolute;
          top: var(--spacing-sm);
          right: var(--spacing-sm);
          background: none;
          border: none;
          color: var(--color-text-secondary);
          font-size: 20px;
          cursor: pointer;
        }

        .detail-grid {
          display: grid;
          grid-template-columns: repeat(2, 1fr);
          gap: var(--spacing-sm);
        }

        .detail-item label {
          display: block;
          font-size: var(--font-size-xs);
          color: var(--color-text-muted);
        }

        .detail-item span {
          font-size: var(--font-size-sm);
          color: var(--color-text-primary);
        }

        .poi-list {
          margin-top: var(--spacing-md);
        }

        .poi-list h5 {
          font-size: var(--font-size-sm);
          margin-bottom: var(--spacing-sm);
        }

        .poi-item {
          display: flex;
          justify-content: space-between;
          padding: var(--spacing-xs) 0;
          border-bottom: 1px solid var(--color-border);
        }

        .status-badge {
          padding: 2px 8px;
          border-radius: var(--radius-sm);
          font-size: var(--font-size-xs);
          color: white;
        }
      `}</style>
    </div>
  );
}

export default TruthTimeline;
