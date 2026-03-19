import React, { useState, useEffect } from 'react';
import {
  getSettings,
  saveSettings,
  storeApiKey,
  startAudioRecording,
  stopAudioRecording,
  type Settings,
} from '../utils/tauri';

type Step = 1 | 2 | 3 | 4 | 5;

interface WizardState {
  currentStep: Step;
  apiKey: string;
  apiKeyError: string;
  shortcut: string;
  micTestStatus: 'idle' | 'testing' | 'success' | 'error';
  micTestMessage: string;
  settings: Settings | null;
  isLoading: boolean;
}

function SetupWizard(): React.ReactElement {
  const [state, setState] = useState<WizardState>({
    currentStep: 1,
    apiKey: '',
    apiKeyError: '',
    shortcut: 'Ctrl+Space',
    micTestStatus: 'idle',
    micTestMessage: '',
    settings: null,
    isLoading: false,
  });

  useEffect(() => {
    getSettings()
      .then((settings) => {
        setState((prev) => ({
          ...prev,
          settings,
          shortcut: settings.shortcut || 'Ctrl+Space',
        }));
      })
      .catch((error) => {
        console.error('Failed to load settings:', error);
      });
  }, []);

  const nextStep = (): void => {
    if (state.currentStep < 5) {
      setState((prev) => ({ ...prev, currentStep: (prev.currentStep + 1) as Step }));
    }
  };

  const prevStep = (): void => {
    if (state.currentStep > 1) {
      setState((prev) => ({ ...prev, currentStep: (prev.currentStep - 1) as Step }));
    }
  };

  const validateApiKey = (key: string): boolean => {
    return key.startsWith('gsk_') && key.length >= 20;
  };

  const handleApiKeySubmit = async (): Promise<void> => {
    if (!validateApiKey(state.apiKey)) {
      setState((prev) => ({
        ...prev,
        apiKeyError: 'Format de clé API invalide. Les clés Groq commencent par "gsk_"',
      }));
      return;
    }

    setState((prev) => ({ ...prev, isLoading: true, apiKeyError: '' }));

    try {
      await storeApiKey(state.apiKey);
      nextStep();
    } catch (error) {
      setState((prev) => ({
        ...prev,
        apiKeyError: 'Échec de l\'enregistrement de la clé. Veuillez réessayer.',
      }));
    } finally {
      setState((prev) => ({ ...prev, isLoading: false }));
    }
  };

  const handleMicTest = async (): Promise<void> => {
    setState((prev) => ({
      ...prev,
      micTestStatus: 'testing',
      micTestMessage: 'Test du microphone...',
    }));

    try {
      await startAudioRecording();
      await new Promise((resolve) => setTimeout(resolve, 2000));
      await stopAudioRecording();

      setState((prev) => ({
        ...prev,
        micTestStatus: 'success',
        micTestMessage: 'Test réussi ! Audio capturé.',
      }));
    } catch (error) {
      setState((prev) => ({
        ...prev,
        micTestStatus: 'error',
        micTestMessage: 'Échec du test. Vérifiez les permissions du microphone.',
      }));
    }
  };

  const handleComplete = async (): Promise<void> => {
    setState((prev) => ({ ...prev, isLoading: true }));

    try {
      const currentSettings = state.settings || (await getSettings());
      await saveSettings({
        ...currentSettings,
        shortcut: state.shortcut,
        setup_completed: true,
      });
      window.location.reload();
    } catch (error) {
      console.error('Failed to complete setup:', error);
      setState((prev) => ({ ...prev, isLoading: false }));
    }
  };

  const renderStepIndicator = (): React.ReactElement => {
    const steps = [
      { num: 1, label: 'Accueil' },
      { num: 2, label: 'Clé API' },
      { num: 3, label: 'Microphone' },
      { num: 4, label: 'Raccourci' },
      { num: 5, label: 'Terminé' },
    ];

    return (
      <div className="flex items-center justify-between mb-8">
        {steps.map((step, index) => (
          <React.Fragment key={step.num}>
            <div className="flex flex-col items-center">
              <div
                className={`w-10 h-10 rounded-full flex items-center justify-center font-semibold text-sm transition-colors duration-200 ${
                  state.currentStep >= step.num
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-200 text-gray-500'
                }`}
              >
                {state.currentStep > step.num ? '✓' : step.num}
              </div>
              <span
                className={`text-xs mt-2 ${
                  state.currentStep >= step.num ? 'text-blue-600 font-medium' : 'text-gray-400'
                }`}
              >
                {step.label}
              </span>
            </div>
            {index < steps.length - 1 && (
              <div
                className={`flex-1 h-0.5 mx-2 transition-colors duration-200 ${
                  state.currentStep > step.num ? 'bg-blue-600' : 'bg-gray-200'
                }`}
              />
            )}
          </React.Fragment>
        ))}
      </div>
    );
  };

  const renderStep1 = (): React.ReactElement => (
    <div className="text-center py-8">
      <div className="w-20 h-20 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-6">
        <svg
          className="w-10 h-10 text-blue-600"
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
      <h2 className="text-2xl font-bold text-gray-800 mb-4">Bienvenue dans Murmure</h2>
      <p className="text-gray-600 mb-2 max-w-md mx-auto">
        Votre assistant de dictée vocale. Parlez naturellement et laissez l'IA transcrire vos pensées
        dans n'importe quelle application.
      </p>
      <p className="text-gray-500 text-sm mb-8 max-w-md mx-auto">
        Cette configuration rapide vous guidera pour configurer votre clé API, tester votre
        microphone et définir votre raccourci clavier.
      </p>
      <button
        onClick={nextStep}
        className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-8 rounded-lg transition-colors duration-200"
      >
        Commencer
      </button>
    </div>
  );

  const renderStep2 = (): React.ReactElement => (
    <div className="py-4">
      <div className="flex items-center mb-6">
        <div className="w-12 h-12 bg-purple-100 rounded-full flex items-center justify-center mr-4">
          <svg
            className="w-6 h-6 text-purple-600"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"
            />
          </svg>
        </div>
        <div>
          <h2 className="text-xl font-bold text-gray-800">Entrez votre clé API</h2>
          <p className="text-gray-500 text-sm">Murmure utilise Groq pour une transcription rapide</p>
        </div>
      </div>

      <div className="bg-gray-50 rounded-lg p-4 mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Clé API Groq
        </label>
        <input
          type="password"
          value={state.apiKey}
          onChange={(e) =>
            setState((prev) => ({ ...prev, apiKey: e.target.value, apiKeyError: '' }))
          }
          placeholder="gsk_..."
          className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
        />
        {state.apiKeyError && (
          <p className="text-red-500 text-sm mt-2">{state.apiKeyError}</p>
        )}
        <p className="text-gray-500 text-xs mt-2">
          Votre clé API est stockée en toute sécurité sur votre appareil. Obtenez votre clé sur{' '}
          <a
            href="https://console.groq.com/keys"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 hover:underline"
          >
            console.groq.com
          </a>
        </p>
      </div>

      <div className="flex justify-between">
        <button
          onClick={prevStep}
          className="text-gray-600 hover:text-gray-800 font-medium py-3 px-6 transition-colors"
        >
          Retour
        </button>
        <button
          onClick={handleApiKeySubmit}
          disabled={!state.apiKey || state.isLoading}
          className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-medium py-3 px-8 rounded-lg transition-colors duration-200"
        >
          {state.isLoading ? 'Enregistrement...' : 'Continuer'}
        </button>
      </div>
    </div>
  );

  const renderStep3 = (): React.ReactElement => (
    <div className="py-4">
      <div className="flex items-center mb-6">
        <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center mr-4">
          <svg
            className="w-6 h-6 text-green-600"
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
          <h2 className="text-xl font-bold text-gray-800">Testez votre microphone</h2>
          <p className="text-gray-500 text-sm">Vérifiez que votre microphone fonctionne</p>
        </div>
      </div>

      <div className="bg-gray-50 rounded-lg p-6 mb-6 text-center">
        {state.micTestStatus === 'idle' && (
          <>
            <p className="text-gray-600 mb-4">
              Cliquez sur le bouton ci-dessous pour tester votre microphone. Nous enregistrerons pendant 2 secondes.
            </p>
            <button
              onClick={handleMicTest}
              className="bg-green-600 hover:bg-green-700 text-white font-medium py-3 px-8 rounded-lg transition-colors duration-200"
            >
              Tester le microphone
            </button>
          </>
        )}

        {state.micTestStatus === 'testing' && (
          <div className="py-4">
            <div className="w-16 h-16 border-4 border-green-200 border-t-green-600 rounded-full animate-spin mx-auto mb-4" />
            <p className="text-gray-600">{state.micTestMessage}</p>
          </div>
        )}

        {state.micTestStatus === 'success' && (
          <div className="py-4">
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg
                className="w-8 h-8 text-green-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <p className="text-green-600 font-medium">{state.micTestMessage}</p>
            <button
              onClick={handleMicTest}
              className="text-blue-600 hover:text-blue-700 text-sm mt-4 underline"
            >
              Tester à nouveau
            </button>
          </div>
        )}

        {state.micTestStatus === 'error' && (
          <div className="py-4">
            <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg
                className="w-8 h-8 text-red-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </div>
            <p className="text-red-600 font-medium">{state.micTestMessage}</p>
            <button
              onClick={handleMicTest}
              className="text-blue-600 hover:text-blue-700 text-sm mt-4 underline"
            >
              Réessayer
            </button>
          </div>
        )}
      </div>

      <div className="flex justify-between">
        <button
          onClick={prevStep}
          className="text-gray-600 hover:text-gray-800 font-medium py-3 px-6 transition-colors"
        >
          Retour
        </button>
        <button
          onClick={nextStep}
          className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-8 rounded-lg transition-colors duration-200"
        >
          Continuer
        </button>
      </div>
    </div>
  );

  const renderStep4 = (): React.ReactElement => (
    <div className="py-4">
      <div className="flex items-center mb-6">
        <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center mr-4">
          <svg
            className="w-6 h-6 text-orange-600"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
            />
          </svg>
        </div>
        <div>
          <h2 className="text-xl font-bold text-gray-800">Configurer le raccourci</h2>
          <p className="text-gray-500 text-sm">Choisissez comment activer la dictée vocale</p>
        </div>
      </div>

      <div className="bg-gray-50 rounded-lg p-6 mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Raccourci d'activation
        </label>
        <div className="flex items-center space-x-4">
          <input
            type="text"
            value={state.shortcut}
            onChange={(e) => setState((prev) => ({ ...prev, shortcut: e.target.value }))}
            className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
            placeholder="ex: Ctrl+Space"
          />
        </div>
        <p className="text-gray-500 text-xs mt-2">
          Appuyez sur ce raccourci n'importe où pour démarrer la dictée vocale. Par défaut : Ctrl+Space.
        </p>
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
        <div className="flex items-start">
          <svg
            className="w-5 h-5 text-blue-600 mt-0.5 mr-3 flex-shrink-0"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <div>
            <p className="text-blue-800 text-sm font-medium">Comment ça marche</p>
            <p className="text-blue-600 text-sm mt-1">
              Appuyez sur {state.shortcut} pour ouvrir l'overlay de dictée. Parlez votre texte, puis
              appuyez à nouveau sur le raccourci ou attendez le silence pour transcrire.
            </p>
          </div>
        </div>
      </div>

      <div className="flex justify-between">
        <button
          onClick={prevStep}
          className="text-gray-600 hover:text-gray-800 font-medium py-3 px-6 transition-colors"
        >
          Retour
        </button>
        <button
          onClick={nextStep}
          className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-8 rounded-lg transition-colors duration-200"
        >
          Continuer
        </button>
      </div>
    </div>
  );

  const renderStep5 = (): React.ReactElement => (
    <div className="text-center py-8">
      <div className="w-20 h-20 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-6">
        <svg
          className="w-10 h-10 text-green-600"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      </div>
      <h2 className="text-2xl font-bold text-gray-800 mb-4">C'est prêt !</h2>
      <p className="text-gray-600 mb-2 max-w-md mx-auto">
        Murmure est maintenant configuré et prêt à l'emploi. Votre assistant de dictée vocale n'est
        qu'à un raccourci.
      </p>

      <div className="bg-gray-50 rounded-lg p-4 my-6 max-w-sm mx-auto text-left">
        <h3 className="text-sm font-semibold text-gray-700 mb-3">Résumé de la configuration</h3>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span className="text-gray-500">Clé API :</span>
            <span className="text-green-600 font-medium">✓ Enregistrée</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-500">Microphone :</span>
            <span className="text-green-600 font-medium">✓ Testé</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-500">Raccourci :</span>
            <span className="text-gray-800 font-medium">{state.shortcut}</span>
          </div>
        </div>
      </div>

      <button
        onClick={handleComplete}
        disabled={state.isLoading}
        className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-medium py-3 px-8 rounded-lg transition-colors duration-200"
      >
        {state.isLoading ? 'Démarrage...' : 'Commencer à utiliser Murmure'}
      </button>
    </div>
  );

  const renderCurrentStep = (): React.ReactElement => {
    switch (state.currentStep) {
      case 1:
        return renderStep1();
      case 2:
        return renderStep2();
      case 3:
        return renderStep3();
      case 4:
        return renderStep4();
      case 5:
        return renderStep5();
      default:
        return renderStep1();
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-lg p-8 max-w-2xl mx-auto">
      {renderStepIndicator()}
      {renderCurrentStep()}
    </div>
  );
}

export default SetupWizard;