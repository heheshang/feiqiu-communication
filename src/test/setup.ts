import { vi } from 'vitest';
import '@testing-library/jest-dom';
import { JSDOM } from 'jsdom';

// Manually initialize jsdom if not already initialized
if (typeof document === 'undefined' || typeof window === 'undefined') {
  const dom = new JSDOM('<!DOCTYPE html><html><body></body></html>', {
    url: 'http://localhost',
  });

  globalThis.window = dom.window as any;
  globalThis.document = dom.window.document as any;
  globalThis.navigator = dom.window.navigator as any;
}

// Setup test environment
globalThis.console = {
  ...console,
  error: vi.fn(),
  warn: vi.fn(),
};

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});
