import { useState, useEffect } from 'react';
import { estimateDuration } from '../utils/time';
import { Wand2, AlertCircle, CheckCircle } from 'lucide-react';

interface ElasticScriptProps {
  initialText: string;
  sceneDuration: number; // in seconds
  onTextChange?: (text: string) => void;
}

export function ElasticScript({ initialText, sceneDuration, onTextChange }: ElasticScriptProps) {
  const [text, setText] = useState(initialText);
  const [estimatedTime, setEstimatedTime] = useState(0);

  useEffect(() => {
    const duration = estimateDuration(text);
    setEstimatedTime(duration);
    onTextChange?.(text);
  }, [text, onTextChange]);

  const status =
    estimatedTime <= sceneDuration
      ? 'safe'
      : estimatedTime <= sceneDuration + 1.5
        ? 'tight'
        : 'overflow';

  const handleCondense = () => {
    // Mock AI Condense
    const words = text.split(' ');
    const shortened = words.slice(0, Math.floor(words.length * 0.75)).join(' ') + '.';
    setText(shortened);
  };

  return (
    <div className="bg-[var(--color-bg-secondary)] p-4 rounded-xl border border-[var(--color-border)] text-[var(--color-text-primary)] font-sans shadow-lg">
      <div className="flex justify-between items-center mb-2">
        <h3 className="font-bold text-[var(--color-text-secondary)] text-sm uppercase tracking-wider flex items-center gap-2">
          <Wand2 size={14} className="text-[var(--color-accent-secondary)]" />
          Combustion Script Editor
        </h3>
        <span
          className={`text-xs font-mono px-2 py-1 rounded-md font-bold transition-colors border ${
            status === 'safe'
              ? 'bg-green-500/20 text-green-400 border-green-500/30'
              : status === 'tight'
                ? 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30'
                : 'bg-red-500/20 text-red-400 border-red-500/30'
          }`}
        >
          {estimatedTime.toFixed(1)}s / {sceneDuration}s
        </span>
      </div>

      {/* Visual Tracks */}
      <div className="relative h-8 bg-[var(--color-bg-tertiary)] rounded mb-4 overflow-hidden flex flex-col gap-1 p-1 border border-[var(--color-border)]">
        {/* Video Track Reference */}
        <div className="h-1/2 w-full bg-[var(--color-accent-primary)]/20 rounded-sm relative">
          <span className="absolute left-1 top-0 text-[8px] text-[var(--color-accent-primary)] font-bold tracking-wider">
            VIDEO SCENE ({sceneDuration}s)
          </span>
        </div>

        {/* Audio Duration Bar */}
        <div className="h-1/2 bg-[var(--color-bg-primary)]/50 rounded-sm relative w-full overflow-hidden">
          <div
            className={`h-full transition-all duration-300 ${
              status === 'safe'
                ? 'bg-green-500/60'
                : status === 'tight'
                  ? 'bg-yellow-500/60'
                  : 'bg-red-500/60'
            }`}
            style={{ width: `${Math.min((estimatedTime / sceneDuration) * 100, 100)}%` }}
          />
          <span className="absolute left-1 top-0 text-[8px] text-[var(--color-text-secondary)] font-bold opacity-75">
            NARRATION
          </span>
        </div>
      </div>

      <textarea
        className="w-full bg-[var(--color-bg-tertiary)] text-[var(--color-text-primary)] p-3 rounded-lg border border-[var(--color-border)] outline-none focus:border-[var(--color-accent-primary)] font-serif text-lg leading-relaxed resize-none h-32 transition-colors placeholder:text-[var(--color-text-muted)]"
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="Write your narration script here..."
      />

      <div className="flex justify-between mt-3 mb-1">
        <div className="flex items-center gap-2 text-xs">
          {status === 'overflow' && (
            <>
              <AlertCircle size={14} className="text-red-500" />
              <span className="text-red-400">Text is too long for this scene.</span>
            </>
          )}
          {status === 'safe' && (
            <>
              <CheckCircle size={14} className="text-green-500" />
              <span className="text-green-500">Perfect fit.</span>
            </>
          )}
          {status === 'tight' && (
            <span className="text-yellow-500 flex items-center gap-1">Fitting, but close.</span>
          )}
        </div>

        {status === 'overflow' && (
          <button
            onClick={handleCondense}
            className="flex items-center gap-2 text-xs bg-[var(--color-accent-primary)] hover:bg-[var(--color-accent-hover)] text-white px-3 py-1.5 rounded-full transition-colors shadow-lg shadow-blue-900/20"
          >
            <Wand2 size={12} />
            Auto-Condense
          </button>
        )}
      </div>
    </div>
  );
}
