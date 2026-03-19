import React, { useEffect, useState } from 'react';
import type { AppState } from '../utils/tauri';

interface OverlayProps {
  state: AppState;
}

function Overlay({ state }: OverlayProps): React.ReactElement | null {
  const [displayDuration, setDisplayDuration] = useState<number>(0);

  // Update timer when recording
  useEffect(() => {
    if (state.type === 'Recording') {
      setDisplayDuration(state.data.duration_ms);
      const interval = setInterval(() => {
        setDisplayDuration((prev) => prev + 1000);
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [state]);

  // Format duration as MM:SS
  const formatDuration = (ms: number): string => {
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  };

  // Hidden state - don't render
  if (state.type === 'Idle') {
    return null;
  }

  // Recording state
  if (state.type === 'Recording') {
    return (
      <div className="fixed inset-0 flex items-center justify-center z-50 animate-fade-in">
        <div className="bg-gray-900/80 backdrop-blur-sm rounded-2xl px-8 py-6 shadow-2xl border border-gray-700/50">
          <div className="flex items-center space-x-4">
            {/* Pulsing red recording dot */}
            <div className="relative">
              <div className="w-4 h-4 bg-red-500 rounded-full animate-pulse" />
              <div className="absolute inset-0 w-4 h-4 bg-red-500 rounded-full animate-ping opacity-75" />
            </div>
            <span className="text-white font-medium text-lg">Enregistrement...</span>
            <span className="text-gray-300 font-mono text-lg">
              {formatDuration(displayDuration)}
            </span>
          </div>
        </div>
      </div>
    );
  }

  // Processing state
  if (state.type === 'Processing') {
    return (
      <div className="fixed inset-0 flex items-center justify-center z-50 animate-fade-in">
        <div className="bg-gray-900/80 backdrop-blur-sm rounded-2xl px-8 py-6 shadow-2xl border border-gray-700/50">
          <div className="flex items-center space-x-4">
            {/* Spinner */}
            <div className="w-6 h-6 border-3 border-gray-600 border-t-blue-500 rounded-full animate-spin" />
            <span className="text-white font-medium text-lg">Transcription...</span>
          </div>
        </div>
      </div>
    );
  }

  // Success state
  if (state.type === 'Success') {
    const text = state.data.text;
    const truncatedText = text.length > 60 ? text.slice(0, 60) + '...' : text;

    return (
      <div className="fixed inset-0 flex items-center justify-center z-50 animate-fade-in">
        <div className="bg-gray-900/80 backdrop-blur-sm rounded-2xl px-8 py-6 shadow-2xl border border-green-500/30 max-w-md">
          <div className="flex flex-col items-center space-y-3">
            {/* Green checkmark */}
            <div className="w-12 h-12 bg-green-500/20 rounded-full flex items-center justify-center">
              <svg
                className="w-7 h-7 text-green-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2.5}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <span className="text-green-400 font-medium">Transcrit !</span>
            <p className="text-gray-300 text-sm text-center max-w-xs break-words">
              &ldquo;{truncatedText}&rdquo;
            </p>
          </div>
        </div>
      </div>
    );
  }

  // Error state
  if (state.type === 'Error') {
    return (
      <div className="fixed inset-0 flex items-center justify-center z-50 animate-fade-in">
        <div className="bg-gray-900/80 backdrop-blur-sm rounded-2xl px-8 py-6 shadow-2xl border border-red-500/30 max-w-md">
          <div className="flex flex-col items-center space-y-3">
            {/* Red X icon */}
            <div className="w-12 h-12 bg-red-500/20 rounded-full flex items-center justify-center">
              <svg
                className="w-7 h-7 text-red-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2.5}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </div>
            <span className="text-red-400 font-medium">Erreur</span>
            <p className="text-gray-300 text-sm text-center max-w-xs">
              {state.data.message}
            </p>
          </div>
        </div>
      </div>
    );
  }

  // Fallback - should never reach here
  return null;
}

export default Overlay;
