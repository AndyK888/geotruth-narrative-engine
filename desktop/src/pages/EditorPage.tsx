import { useState } from 'react';
import { MomentCatcher } from '../components/MomentCatcher';
import { ElasticScript } from '../components/ElasticScript';
import { TruthTimelinePro } from '../components/TruthTimelinePro';
import { TruthVerification } from '../components/TruthVerification';
import { CheckCircle2, ChevronLeft, FileCode, Camera, Trash2, Clock } from 'lucide-react';

interface EditorPageProps {
  videoPath: string;
  onBack?: () => void;
}

interface CapturedMoment {
  id: string;
  timestamp: number;
  image: string;
  description?: string;
}

export function EditorPage({ videoPath, onBack }: EditorPageProps) {
  const [activeTab, setActiveTab] = useState<'create' | 'script' | 'verify'>('create');
  const [currentTime, _setCurrentTime] = useState(0);
  const [capturedMoments, setCapturedMoments] = useState<CapturedMoment[]>([]);

  // Mock Data
  const [events, setEvents] = useState([
    { id: '1', action: 'gps', startTime: 5, endTime: 15, text: 'Big Sur View' },
    { id: '2', action: 'gps', startTime: 25, endTime: 30, text: 'Bixby Bridge' },
  ]);

  const [narration, _setNarration] = useState([
    {
      id: 'n1',
      action: 'narration',
      startTime: 6,
      endTime: 14,
      text: 'Looking out over the coastline...',
    },
  ]);

  // Derived events for verification
  const verificationEvents = [
    ...events.map((e) => ({
      id: e.id,
      timestamp: e.startTime,
      type: 'gps' as const,
      content: e.text,
      verified: true,
    })),
    ...narration.map((e) => ({
      id: e.id,
      timestamp: e.startTime,
      type: 'narration' as const,
      content: e.text || '',
      verified: false,
    })),
  ].sort((a, b) => a.timestamp - b.timestamp);

  const handleMomentCaptured = (data: {
    timestamp: number;
    image: string;
    description?: string;
  }) => {
    const newMoment: CapturedMoment = {
      id: Date.now().toString(),
      timestamp: data.timestamp,
      image: data.image,
      description: data.description || 'Analyzing scene...',
    };
    setCapturedMoments([newMoment, ...capturedMoments]);

    // Also add to timeline events for now
    const newEvent = {
      id: Date.now().toString(),
      action: 'gps',
      startTime: data.timestamp,
      endTime: data.timestamp + 5,
      text: data.description || 'Captured Moment',
    };
    setEvents([...events, newEvent]);
  };

  const handleAutoCaptured = (
    moments: Array<{ timestamp: number; image: string; description?: string }>
  ) => {
    const newMoments = moments.map((m) => ({
      id: Date.now().toString() + Math.random().toString().substr(2, 5),
      timestamp: m.timestamp,
      image: m.image,
      description: m.description || 'Auto-detected scene',
    }));

    const newEvents = moments.map((m, i) => ({
      id: Date.now().toString() + i,
      action: 'gps',
      startTime: m.timestamp,
      endTime: m.timestamp + 5,
      text: m.description || 'Auto-detected scene',
    }));

    setCapturedMoments((prev) => [...newMoments, ...prev]);
    setEvents((prev) => [...prev, ...newEvents]);
  };

  const deleteMoment = (id: string) => {
    setCapturedMoments((moments) => moments.filter((m) => m.id !== id));
  };

  const formatTime = (seconds: number) => {
    const date = new Date(seconds * 1000);
    return date.toISOString().substr(14, 5);
  };

  return (
    <div className="app h-screen bg-[var(--color-bg-primary)] overflow-hidden flex flex-col relative">
      {/* Background Effects */}
      <div className="absolute inset-0 pointer-events-none z-0">
        <div className="absolute top-[-20%] right-[-10%] w-[50%] h-[50%] bg-purple-900/10 rounded-full blur-[120px]" />
        <div className="absolute bottom-[-20%] left-[-10%] w-[50%] h-[50%] bg-blue-900/10 rounded-full blur-[120px]" />
      </div>

      {/* Header */}
      <header className="app-header z-10 shrink-0">
        <div className="flex items-center gap-4">
          <button
            onClick={onBack}
            className="p-2 rounded-full hover:bg-[var(--color-bg-glass-hover)] text-[var(--color-text-secondary)] hover:text-white transition-all"
          >
            <ChevronLeft size={20} />
          </button>
          <div>
            <h1 className="text-lg font-bold bg-[var(--color-accent-gradient)] bg-clip-text text-transparent">
              Narrative Editor
            </h1>
            <div className="text-xs text-[var(--color-text-secondary)] flex items-center gap-2">
              <span className="opacity-50">EDITING:</span>
              <span className="font-mono text-[var(--color-accent-primary)]">
                {videoPath.split('/').pop()}
              </span>
            </div>
          </div>
        </div>
        {/* Styled Segmented Control */}
        <div className="bg-[var(--color-bg-secondary)] p-1 rounded-lg flex items-center border border-[var(--color-border)]">
          <button
            onClick={() => setActiveTab('create')}
            className={`
                            px-4 py-1.5 rounded-md text-sm font-medium flex items-center gap-2 transition-all
                            ${
                              activeTab === 'create'
                                ? 'bg-[var(--color-bg-tertiary)] text-white shadow-sm border border-[var(--color-border-hover)]'
                                : 'text-[var(--color-text-secondary)] hover:text-white hover:bg-[var(--color-bg-glass-hover)]'
                            }
                        `}
          >
            <Camera size={14} className={activeTab === 'create' ? 'text-blue-400' : ''} />
            Moment Catcher
          </button>
          <button
            onClick={() => setActiveTab('script')}
            className={`
                            px-4 py-1.5 rounded-md text-sm font-medium flex items-center gap-2 transition-all
                            ${
                              activeTab === 'script'
                                ? 'bg-[var(--color-bg-tertiary)] text-white shadow-sm border border-[var(--color-border-hover)]'
                                : 'text-[var(--color-text-secondary)] hover:text-white hover:bg-[var(--color-bg-glass-hover)]'
                            }
                        `}
          >
            <FileCode size={14} className={activeTab === 'script' ? 'text-purple-400' : ''} />
            Elastic Truth
          </button>
          <button
            onClick={() => setActiveTab('verify')}
            className={`
                            px-4 py-1.5 rounded-md text-sm font-medium flex items-center gap-2 transition-all
                            ${
                              activeTab === 'verify'
                                ? 'bg-[var(--color-bg-tertiary)] text-white shadow-sm border border-[var(--color-border-hover)]'
                                : 'text-[var(--color-text-secondary)] hover:text-white hover:bg-[var(--color-bg-glass-hover)]'
                            }
                        `}
          >
            <CheckCircle2 size={14} className={activeTab === 'verify' ? 'text-green-400' : ''} />
            Verification
          </button>
        </div>
        <div className="w-32"></div> {/* Spacer for balance */}
      </header>

      {/* Main Workspace */}
      <main className="flex-1 overflow-hidden relative z-10 p-6">
        {/* MODE 1: MOMENT CATCHER */}
        {activeTab === 'create' && (
          <div className="h-full grid grid-cols-12 gap-6 animate-in fade-in zoom-in-95 duration-300">
            {/* Left Panel: Video Player */}
            <div className="col-span-8 flex flex-col gap-4">
              <div className="flex-1 bg-black rounded-xl overflow-hidden shadow-2xl border border-[var(--color-border)] relative group">
                <MomentCatcher
                  videoPath={videoPath}
                  onMomentCaptured={handleMomentCaptured}
                  onAutoCaptured={handleAutoCaptured}
                  moments={capturedMoments}
                />
              </div>
            </div>

            {/* Right Panel: Captured Moments Stream */}
            <div className="col-span-4 flex flex-col gap-4 min-w-0">
              <div className="flex items-center justify-between px-2">
                <h3 className="font-bold text-[var(--color-text-primary)] flex items-center gap-2">
                  <Clock size={16} className="text-[var(--color-accent-secondary)]" />
                  Captured Moments
                </h3>
                <span className="text-xs bg-[var(--color-bg-tertiary)] px-2 py-1 rounded text-[var(--color-text-secondary)]">
                  {capturedMoments.length} items
                </span>
              </div>

              <div className="flex-1 overflow-y-auto overflow-x-hidden pr-2 space-y-3 custom-scrollbar">
                {capturedMoments.length === 0 ? (
                  <div className="h-40 flex flex-col items-center justify-center border-2 border-dashed border-[var(--color-border)] rounded-xl text-[var(--color-text-muted)]">
                    <Camera size={24} className="mb-2 opacity-50" />
                    <p className="text-sm">No moments captured yet.</p>
                    <p className="text-xs opacity-50 mt-1">Press 'C' or click Capture</p>
                  </div>
                ) : (
                  capturedMoments.map((moment) => (
                    <div
                      key={moment.id}
                      className="bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded-lg p-3 hover:border-[var(--color-accent-primary)] transition-all group"
                    >
                      <div className="flex gap-3">
                        <div className="w-24 h-16 bg-black rounded shrink-0 overflow-hidden relative">
                          <img
                            src={moment.image}
                            alt="Capture"
                            className="w-full h-full object-cover opacity-80 group-hover:opacity-100 transition-opacity"
                          />
                          <span className="absolute bottom-1 right-1 bg-black/70 text-[10px] text-white px-1 rounded font-mono">
                            {formatTime(moment.timestamp)}
                          </span>
                        </div>
                        <div className="flex-1 min-w-0 flex flex-col justify-between overflow-hidden">
                          <div>
                            <p className="text-sm text-[var(--color-text-primary)] line-clamp-2 leading-tight break-words">
                              {moment.description}
                            </p>
                          </div>
                          <div className="flex justify-between items-center mt-2">
                            <div className="flex items-center gap-1">
                              <div className="w-2 h-2 rounded-full bg-green-500 shrink-0"></div>
                              <span className="text-[10px] text-[var(--color-text-secondary)] whitespace-nowrap">
                                GPS Locked
                              </span>
                            </div>
                            <button
                              onClick={() => deleteMoment(moment.id)}
                              className="text-[var(--color-text-muted)] hover:text-red-400 p-1 rounded-md hover:bg-[var(--color-bg-tertiary)] transition-colors shrink-0"
                              title="Delete Moment"
                            >
                              <Trash2 size={12} />
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>
        )}

        {/* MODE 2: ELASTIC SCRIPT */}
        {activeTab === 'script' && (
          <div className="h-full flex flex-col gap-6 animate-in fade-in zoom-in-95 duration-300">
            <div className="flex gap-6 h-2/3">
              <div className="w-2/3 bg-black rounded-xl overflow-hidden border border-[var(--color-border)] shadow-xl relative">
                {/* Context Video Player */}
                {/* Reusing MomentCatcher as the player for now, but disabling capture controls via props if we updated it */}
                <MomentCatcher videoPath={videoPath} moments={capturedMoments} />
              </div>
              <div className="w-1/3 flex flex-col gap-4 overflow-y-auto pr-2">
                <h3 className="font-bold text-[var(--color-text-secondary)] text-xs uppercase tracking-wider mb-2">
                  Narrative Blocks
                </h3>
                <ElasticScript
                  initialText="The rugged coastline of Big Sur offers some of the most dramatic views in California. Highway 1 winds its way along the cliffs."
                  sceneDuration={10}
                  onTextChange={(t) => console.log(t)}
                />
                <ElasticScript
                  initialText="Bixby Creek Bridge is one of the most photographed bridges in California."
                  sceneDuration={5}
                  onTextChange={(t) => console.log(t)}
                />
                <div className="border-2 border-dashed border-[var(--color-border)] rounded-xl p-4 flex items-center justify-center text-[var(--color-text-muted)] cursor-pointer hover:bg-[var(--color-bg-glass-hover)] hover:text-[var(--color-text-primary)] transition-all">
                  + Add Script Block
                </div>
              </div>
            </div>

            <div className="h-1/3 bg-[var(--color-bg-secondary)] rounded-xl border border-[var(--color-border)] overflow-hidden">
              <TruthTimelinePro
                duration={60}
                events={events}
                narrationTracks={narration}
                currentTime={currentTime}
              />
            </div>
          </div>
        )}

        {/* MODE 3: VERIFICATION */}
        {activeTab === 'verify' && (
          <div className="h-full animate-in fade-in zoom-in-95 duration-300 bg-[var(--color-bg-secondary)] rounded-xl border border-[var(--color-border)] overflow-hidden shadow-2xl">
            <TruthVerification videoPath={videoPath} events={verificationEvents} />
          </div>
        )}
      </main>
    </div>
  );
}

// Helper Style
const style = document.createElement('style');
style.textContent = `
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--color-bg-tertiary); border-radius: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--color-accent-primary); }
`;
document.head.appendChild(style);
