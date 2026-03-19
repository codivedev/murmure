import { useState, useCallback, useEffect, useRef } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import {
  AppState,
  startAudioRecording,
  stopAudioRecording,
  transcribeAudio,
  insertText,
} from '../utils/tauri';

export interface UseRecordingReturn {
  state: AppState;
  transcription: string | null;
  error: string | null;
  start: () => Promise<void>;
  stop: () => Promise<void>;
}

export function useRecording(): UseRecordingReturn {
  const [state, setState] = useState<AppState>({ type: 'Idle' });
  const [transcription, setTranscription] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const recordingStartTimeRef = useRef<number | null>(null);
  const durationIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const resetTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const unlistenPressRef = useRef<UnlistenFn | null>(null);
  const unlistenReleaseRef = useRef<UnlistenFn | null>(null);

  const clearTimers = useCallback(() => {
    if (durationIntervalRef.current) {
      clearInterval(durationIntervalRef.current);
      durationIntervalRef.current = null;
    }
    if (resetTimeoutRef.current) {
      clearTimeout(resetTimeoutRef.current);
      resetTimeoutRef.current = null;
    }
  }, []);

  const scheduleReset = useCallback((delayMs: number) => {
    clearTimers();
    resetTimeoutRef.current = setTimeout(() => {
      setState({ type: 'Idle' });
      setTranscription(null);
      setError(null);
    }, delayMs);
  }, [clearTimers]);

  const start = useCallback(async () => {
    if (state.type === 'Recording' || state.type === 'Processing') {
      return;
    }

    try {
      clearTimers();
      setTranscription(null);
      setError(null);

      await startAudioRecording();

      recordingStartTimeRef.current = Date.now();
      setState({ type: 'Recording', data: { duration_ms: 0 } });

      durationIntervalRef.current = setInterval(() => {
        if (recordingStartTimeRef.current) {
          const durationMs = Date.now() - recordingStartTimeRef.current;
          setState({ type: 'Recording', data: { duration_ms: durationMs } });
        }
      }, 100);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to start recording';
      setError(errorMessage);
      setState({ type: 'Error', data: { message: errorMessage } });
      scheduleReset(5000);
    }
  }, [state.type, clearTimers, scheduleReset]);

  const stop = useCallback(async () => {
    if (state.type !== 'Recording') {
      return;
    }

    try {
      clearTimers();
      setState({ type: 'Processing' });

      const wavBytes = await stopAudioRecording();
      const result = await transcribeAudio(wavBytes);

      setTranscription(result.text);
      await insertText(result.text);

      setState({ type: 'Success', data: { text: result.text } });
      scheduleReset(2000);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to process recording';
      setError(errorMessage);
      setState({ type: 'Error', data: { message: errorMessage } });
      scheduleReset(5000);
    }
  }, [state.type, clearTimers, scheduleReset]);

  useEffect(() => {
    let isMounted = true;

    const setupListeners = async () => {
      try {
        const unlistenPress = await listen('shortcut-pressed', () => {
          if (isMounted) {
            start();
          }
        });
        unlistenPressRef.current = unlistenPress;

        const unlistenRelease = await listen('shortcut-released', () => {
          if (isMounted) {
            stop();
          }
        });
        unlistenReleaseRef.current = unlistenRelease;
      } catch (err) {
        console.error('Failed to set up shortcut listeners:', err);
      }
    };

    setupListeners();

    return () => {
      isMounted = false;
      clearTimers();

      if (unlistenPressRef.current) {
        unlistenPressRef.current();
        unlistenPressRef.current = null;
      }
      if (unlistenReleaseRef.current) {
        unlistenReleaseRef.current();
        unlistenReleaseRef.current = null;
      }
    };
  }, [start, stop, clearTimers]);

  return {
    state,
    transcription,
    error,
    start,
    stop,
  };
}
