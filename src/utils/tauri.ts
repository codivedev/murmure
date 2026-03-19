import { invoke } from '@tauri-apps/api/core';

// Type definitions matching Rust types from Plan 1

export interface OverlayPosition {
  x: number;
  y: number;
}

export interface Settings {
  shortcut: string;
  language: string;
  overlay_position: OverlayPosition;
  setup_completed: boolean;
}

export interface TranscriptionResult {
  text: string;
  duration_ms: number;
  language?: string;
}

export type AppState =
  | { type: 'Idle' }
  | { type: 'Recording'; data: { duration_ms: number } }
  | { type: 'Processing' }
  | { type: 'Success'; data: { text: string } }
  | { type: 'Error'; data: { message: string } };

export interface WindowInfo {
  title: string;
  class: string;
}

// Plan 1 Commands (Implemented in Rust)

export async function getSettings(): Promise<Settings> {
  return invoke<Settings>('get_settings');
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke<void>('save_settings', { settings });
}

export async function storeApiKey(key: string): Promise<void> {
  return invoke<void>('store_api_key', { key });
}

export async function hasApiKey(): Promise<boolean> {
  return invoke<boolean>('has_api_key');
}

// Plan 2-3 Commands (Mock implementations - not yet implemented in Rust)

export async function startAudioRecording(): Promise<void> {
  // TODO: Replace with real invoke when Plan 2 is complete
  console.log('[MOCK] start_audio_recording called');
  return Promise.resolve();
}

export async function stopAudioRecording(): Promise<Uint8Array> {
  // TODO: Replace with real invoke when Plan 2 is complete
  console.log('[MOCK] stop_audio_recording called');
  // Return empty WAV header (44 bytes minimum)
  const mockWav = new Uint8Array(44);
  return Promise.resolve(mockWav);
}

export async function transcribeAudio(wavBytes: Uint8Array): Promise<TranscriptionResult> {
  // TODO: Replace with real invoke when Plan 3 is complete
  console.log('[MOCK] transcribe_audio called with', wavBytes.length, 'bytes');
  return Promise.resolve({
    text: '[Mock transcription]',
    duration_ms: 1000,
    language: 'en',
  });
}

export async function insertText(text: string): Promise<void> {
  // TODO: Replace with real invoke when Plan 3 is complete
  console.log('[MOCK] insert_text called:', text);
  return Promise.resolve();
}

export async function registerShortcut(shortcut: string): Promise<void> {
  // TODO: Replace with real invoke when Plan 2 is complete
  console.log('[MOCK] register_shortcut called:', shortcut);
  return Promise.resolve();
}

export async function getActiveWindow(): Promise<WindowInfo> {
  // TODO: Replace with real invoke when Plan 3 is complete
  console.log('[MOCK] get_active_window called');
  return Promise.resolve({
    title: '[Mock Window]',
    class: '[mock-class]',
  });
}
