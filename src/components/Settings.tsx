import React, { useState, useEffect } from 'react';
import {
  getSettings,
  saveSettings,
  storeApiKey,
  hasApiKey,
  type Settings,
} from '../utils/tauri';

interface SettingsState {
  shortcut: string;
  language: string;
  hasApiKey: boolean;
  isLoading: boolean;
  saveStatus: 'idle' | 'saving' | 'success' | 'error';
  saveMessage: string;
  showApiKeyInput: boolean;
  newApiKey: string;
  apiKeyError: string;
}

const LANGUAGES = [
  { value: 'auto', label: 'Détection automatique' },
  { value: 'en', label: 'Anglais' },
  { value: 'fr', label: 'Français' },
  { value: 'es', label: 'Espagnol' },
  { value: 'de', label: 'Allemand' },
  { value: 'it', label: 'Italien' },
  { value: 'pt', label: 'Portugais' },
  { value: 'ja', label: 'Japonais' },
  { value: 'zh', label: 'Chinois' },
  { value: 'ko', label: 'Coréen' },
];

const DEFAULT_SETTINGS: Settings = {
  shortcut: 'Ctrl+Space',
  language: 'auto',
  overlay_position: { x: 100, y: 100 },
  setup_completed: true,
};

function Settings(): React.ReactElement {
  const [state, setState] = useState<SettingsState>({
    shortcut: DEFAULT_SETTINGS.shortcut,
    language: DEFAULT_SETTINGS.language,
    hasApiKey: false,
    isLoading: true,
    saveStatus: 'idle',
    saveMessage: '',
    showApiKeyInput: false,
    newApiKey: '',
    apiKeyError: '',
  });

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async (): Promise<void> => {
    try {
      const settings = await getSettings();
      const apiKeyExists = await hasApiKey();
      setState((prev) => ({
        ...prev,
        shortcut: settings.shortcut || DEFAULT_SETTINGS.shortcut,
        language: settings.language || DEFAULT_SETTINGS.language,
        hasApiKey: apiKeyExists,
        isLoading: false,
      }));
    } catch (error) {
      console.error('Failed to load settings:', error);
      setState((prev) => ({
        ...prev,
        isLoading: false,
        saveStatus: 'error',
        saveMessage: 'Échec du chargement des paramètres',
      }));
    }
  };

  const saveAllSettings = async (newShortcut: string, newLanguage: string): Promise<void> => {
    setState((prev) => ({ ...prev, saveStatus: 'saving', saveMessage: '' }));

    try {
      const currentSettings = await getSettings();
      await saveSettings({
        ...currentSettings,
        shortcut: newShortcut,
        language: newLanguage,
      });
      setState((prev) => ({
        ...prev,
        saveStatus: 'success',
        saveMessage: 'Paramètres enregistrés !',
      }));
      setTimeout(() => {
        setState((prev) => ({ ...prev, saveStatus: 'idle', saveMessage: '' }));
      }, 2000);
    } catch (error) {
      console.error('Failed to save settings:', error);
      setState((prev) => ({
        ...prev,
        saveStatus: 'error',
        saveMessage: 'Échec de l\'enregistrement',
      }));
    }
  };

  const handleShortcutChange = async (value: string): Promise<void> => {
    setState((prev) => ({ ...prev, shortcut: value }));
    await saveAllSettings(value, state.language);
  };

  const handleLanguageChange = async (value: string): Promise<void> => {
    setState((prev) => ({ ...prev, language: value }));
    await saveAllSettings(state.shortcut, value);
  };

  const handleApiKeyChange = async (): Promise<void> => {
    if (!state.newApiKey) {
      setState((prev) => ({ ...prev, showApiKeyInput: false }));
      return;
    }

    if (!state.newApiKey.startsWith('gsk_') || state.newApiKey.length < 20) {
      setState((prev) => ({
        ...prev,
        apiKeyError: 'Format de clé invalide. Les clés Groq commencent par "gsk_"',
      }));
      return;
    }

    try {
      await storeApiKey(state.newApiKey);
      setState((prev) => ({
        ...prev,
        hasApiKey: true,
        showApiKeyInput: false,
        newApiKey: '',
        apiKeyError: '',
        saveStatus: 'success',
        saveMessage: 'Clé API mise à jour !',
      }));
      setTimeout(() => {
        setState((prev) => ({ ...prev, saveStatus: 'idle', saveMessage: '' }));
      }, 2000);
    } catch (error) {
      console.error('Failed to store API key:', error);
      setState((prev) => ({
        ...prev,
        apiKeyError: 'Échec de l\'enregistrement. Veuillez réessayer.',
      }));
    }
  };

  const maskApiKey = (): string => {
    if (!state.hasApiKey) return 'Non configurée';
    return 'gsk_••••••••••••••••••••••••••';
  };

  if (state.isLoading) {
    return (
      <div className="bg-gray-800 rounded-lg shadow-lg p-6">
        <div className="flex items-center justify-center py-12">
          <div className="w-8 h-8 border-4 border-gray-600 border-t-blue-500 rounded-full animate-spin" />
          <span className="ml-3 text-gray-300">Chargement...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-xl font-semibold text-white">Paramètres</h2>
          <p className="text-gray-400 text-sm">Gérez vos préférences</p>
        </div>
        {state.saveStatus === 'success' && (
          <div className="flex items-center text-green-400 bg-green-900/50 px-3 py-2 rounded-lg">
            <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
            <span className="text-sm font-medium">{state.saveMessage}</span>
          </div>
        )}
        {state.saveStatus === 'error' && (
          <div className="flex items-center text-red-400 bg-red-900/50 px-3 py-2 rounded-lg">
            <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
            <span className="text-sm font-medium">{state.saveMessage}</span>
          </div>
        )}
      </div>

      <div className="space-y-6">
        {/* Raccourci */}
        <div className="border-b border-gray-700 pb-6">
          <div className="flex items-start">
            <div className="w-10 h-10 bg-orange-900/50 rounded-lg flex items-center justify-center mr-4 flex-shrink-0">
              <svg className="w-5 h-5 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
            <div className="flex-1">
              <h3 className="text-lg font-medium text-white mb-1">Raccourci d'activation</h3>
              <p className="text-gray-400 text-sm mb-3">
                Appuyez sur ce raccourci pour démarrer la dictée
              </p>
              <input
                type="text"
                value={state.shortcut}
                onChange={(e) => handleShortcutChange(e.target.value)}
                className="w-full max-w-xs px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all text-white placeholder-gray-400"
                placeholder="ex: Ctrl+Space"
              />
              <p className="text-gray-500 text-xs mt-2">
                Par défaut : Ctrl+Space
              </p>
            </div>
          </div>
        </div>

        {/* Langue */}
        <div className="border-b border-gray-700 pb-6">
          <div className="flex items-start">
            <div className="w-10 h-10 bg-blue-900/50 rounded-lg flex items-center justify-center mr-4 flex-shrink-0">
              <svg className="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
              </svg>
            </div>
            <div className="flex-1">
              <h3 className="text-lg font-medium text-white mb-1">Langue de transcription</h3>
              <p className="text-gray-400 text-sm mb-3">
                Sélectionnez la langue pour la transcription
              </p>
              <select
                value={state.language}
                onChange={(e) => handleLanguageChange(e.target.value)}
                className="w-full max-w-xs px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all text-white appearance-none cursor-pointer"
                style={{ backgroundColor: '#374151' }}
              >
                {LANGUAGES.map((lang) => (
                  <option key={lang.value} value={lang.value} className="bg-gray-700 text-white">
                    {lang.label}
                  </option>
                ))}
              </select>
            </div>
          </div>
        </div>

        {/* Clé API */}
        <div className="border-b border-gray-700 pb-6">
          <div className="flex items-start">
            <div className="w-10 h-10 bg-purple-900/50 rounded-lg flex items-center justify-center mr-4 flex-shrink-0">
              <svg className="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
              </svg>
            </div>
            <div className="flex-1">
              <h3 className="text-lg font-medium text-white mb-1">Clé API Groq</h3>
              <p className="text-gray-400 text-sm mb-3">
                Votre clé est stockée en toute sécurité
              </p>

              {!state.showApiKeyInput ? (
                <div className="flex items-center space-x-4">
                  <div className="flex-1 max-w-xs px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-gray-300 font-mono text-sm">
                    {maskApiKey()}
                  </div>
                  <button
                    onClick={() => setState((prev) => ({ ...prev, showApiKeyInput: true, apiKeyError: '' }))}
                    className="text-blue-400 hover:text-blue-300 font-medium text-sm transition-colors"
                  >
                    Modifier
                  </button>
                </div>
              ) : (
                <div className="space-y-3">
                  <input
                    type="password"
                    value={state.newApiKey}
                    onChange={(e) =>
                      setState((prev) => ({ ...prev, newApiKey: e.target.value, apiKeyError: '' }))
                    }
                    placeholder="gsk_..."
                    className="w-full max-w-xs px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all text-white placeholder-gray-400"
                  />
                  {state.apiKeyError && (
                    <p className="text-red-400 text-sm">{state.apiKeyError}</p>
                  )}
                  <div className="flex space-x-3">
                    <button
                      onClick={handleApiKeyChange}
                      disabled={!state.newApiKey}
                      className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200 text-sm"
                    >
                      Enregistrer
                    </button>
                    <button
                      onClick={() =>
                        setState((prev) => ({
                          ...prev,
                          showApiKeyInput: false,
                          newApiKey: '',
                          apiKeyError: '',
                        }))
                      }
                      className="text-gray-400 hover:text-gray-300 font-medium py-2 px-4 transition-colors text-sm"
                    >
                      Annuler
                    </button>
                  </div>
                  <p className="text-gray-500 text-xs">
                    Obtenez votre clé sur{' '}
                    <a
                      href="https://console.groq.com/keys"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-blue-400 hover:underline"
                    >
                      console.groq.com
                    </a>
                  </p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* À propos */}
        <div>
          <div className="flex items-start">
            <div className="w-10 h-10 bg-gray-700 rounded-lg flex items-center justify-center mr-4 flex-shrink-0">
              <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div className="flex-1">
              <h3 className="text-lg font-medium text-white mb-1">À propos</h3>
              <div className="space-y-2 text-sm">
                <div className="flex items-center space-x-4">
                  <span className="text-gray-400">Version</span>
                  <span className="text-white font-medium">0.1.0</span>
                </div>
                <div className="flex items-center space-x-4">
                  <span className="text-gray-400">GitHub</span>
                  <a
                    href="https://github.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-blue-400 hover:underline"
                  >
                    github.com/murmure
                  </a>
                </div>
                <div className="flex items-center space-x-4">
                  <span className="text-gray-400">Console Groq</span>
                  <a
                    href="https://console.groq.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-blue-400 hover:underline"
                  >
                    console.groq.com
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Settings;