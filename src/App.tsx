import React, { useState, useEffect } from 'react';
import SetupWizard from './components/SetupWizard';
import SettingsComponent from './components/Settings';
import Overlay from './components/Overlay';
import { useRecording } from './hooks/useRecording';
import { getSettings, registerShortcut, type Settings } from './utils/tauri';

type ViewState = 'loading' | 'setup' | 'main';

function App(): React.ReactElement {
  const [view, setView] = useState<ViewState>('loading');
  const [error, setError] = useState<string | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [showSettings, setShowSettings] = useState(false);

  const { state, transcription, error: recordingError } = useRecording();

  useEffect(() => {
    const checkSetup = async (): Promise<void> => {
      try {
        setError(null);
        const currentSettings = await getSettings();
        setSettings(currentSettings);

        if (!currentSettings.setup_completed) {
          setView('setup');
        } else {
          setView('main');
          await registerShortcut(currentSettings.shortcut);
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Échec de l\'initialisation';
        setError(errorMessage);
        setView('setup');
      }
    };

    checkSetup();
  }, []);

  if (view === 'loading') {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center">
        <div className="flex flex-col items-center space-y-4">
          <div className="w-8 h-8 border-2 border-gray-600 border-t-blue-500 rounded-full animate-spin" />
          <span className="text-gray-400 text-sm">Initialisation...</span>
        </div>
      </div>
    );
  }

  const ErrorBanner = error || recordingError ? (
    <div className="fixed top-4 left-1/2 transform -translate-x-1/2 z-50">
      <div className="bg-red-500/90 text-white px-4 py-2 rounded-lg shadow-lg text-sm">
        {error || recordingError}
      </div>
    </div>
  ) : null;

  if (view === 'setup') {
    return (
      <div className="min-h-screen bg-gray-900">
        {ErrorBanner}
        <div className="py-8">
          <SetupWizard />
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      {ErrorBanner}

      <Overlay state={state} />

      <div className="p-6">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
              <svg
                className="w-5 h-5 text-white"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
                />
              </svg>
            </div>
            <div>
              <h1 className="text-lg font-semibold text-white">Murmure</h1>
              <p className="text-xs text-gray-400">Assistant de dictée vocale</p>
            </div>
          </div>

          <button
            onClick={() => setShowSettings(!showSettings)}
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800 rounded-lg transition-colors"
            title="Paramètres"
          >
            <svg
              className="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
              />
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
              />
            </svg>
          </button>
        </div>

        <div className="bg-gray-800/50 rounded-xl p-4 mb-4 border border-gray-700/50">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div
                className={`w-2 h-2 rounded-full ${
                  state.type === 'Idle'
                    ? 'bg-green-500'
                    : state.type === 'Recording'
                      ? 'bg-red-500 animate-pulse'
                      : state.type === 'Processing'
                        ? 'bg-blue-500 animate-pulse'
                        : state.type === 'Success'
                          ? 'bg-green-500'
                          : 'bg-red-500'
                }`}
              />
              <span className="text-sm text-gray-300">
                {state.type === 'Idle' && 'Prêt à enregistrer'}
                {state.type === 'Recording' && 'Enregistrement...'}
                {state.type === 'Processing' && 'Transcription...'}
                {state.type === 'Success' && 'Transcrit !'}
                {state.type === 'Error' && 'Erreur'}
              </span>
            </div>
            <span className="text-xs text-gray-500 font-mono">
              {settings?.shortcut || 'Ctrl+Space'}
            </span>
          </div>

          {transcription && (
            <div className="mt-3 pt-3 border-t border-gray-700/50">
              <p className="text-xs text-gray-500 mb-1">Dernière transcription :</p>
              <p className="text-sm text-gray-300 truncate">&ldquo;{transcription}&rdquo;</p>
            </div>
          )}
        </div>

        {showSettings && (
          <div className="animate-fade-in">
            <SettingsComponent />
          </div>
        )}

        <div className="mt-6 text-center">
          <p className="text-xs text-gray-500">
            Appuyez sur{' '}
            <span className="px-1.5 py-0.5 bg-gray-800 rounded text-gray-400 font-mono text-xs">
              {settings?.shortcut || 'Ctrl+Space'}
            </span>{' '}
            n'importe où pour commencer la dictée
          </p>
        </div>
      </div>
    </div>
  );
}

export default App;
