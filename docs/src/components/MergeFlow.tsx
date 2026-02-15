import { useState, useEffect } from 'react';

interface Props { lang: 'en' | 'es'; }

const i18n = {
  en: {
    steps: [
      { label: 'Edit Offline', desc: 'Three devices create data independently while disconnected from the network.' },
      { label: 'Go Online', desc: 'Devices reconnect to the network and discover each other.' },
      { label: 'CRDT Sync', desc: 'State vectors are exchanged. CRDTs merge all changes automatically — no central server.' },
      { label: 'Converged!', desc: 'All replicas have the exact same state. Zero conflicts, mathematically guaranteed.' },
    ],
    devices: [
      { name: 'Phone', owner: 'Alice' },
      { name: 'Laptop', owner: 'Bob' },
      { name: 'Sensor', owner: 'Carol' },
    ],
    notes: ['Meeting at 3pm', 'Buy groceries', 'Deploy v2.1'],
    status: ['Offline', 'Online', 'Syncing...', 'Synced'],
    connect: 'Connect All',
    disconnect: 'Disconnect',
    next: 'Next',
    back: 'Back',
    reset: 'Reset',
    synced: 'synced',
    syncMsg: 'Exchanging state vectors between peers...',
    convergedMsg: 'All states are identical — convergence guaranteed',
  },
  es: {
    steps: [
      { label: 'Editar Offline', desc: 'Tres dispositivos crean datos de forma independiente sin conexión a la red.' },
      { label: 'Conectar', desc: 'Los dispositivos se reconectan a la red y se descubren entre sí.' },
      { label: 'Sync CRDT', desc: 'Se intercambian vectores de estado. Los CRDTs fusionan los cambios automáticamente — sin servidor central.' },
      { label: '¡Convergido!', desc: 'Todas las réplicas tienen exactamente el mismo estado. Cero conflictos, garantizado matemáticamente.' },
    ],
    devices: [
      { name: 'Teléfono', owner: 'Alice' },
      { name: 'Laptop', owner: 'Bob' },
      { name: 'Sensor', owner: 'Carol' },
    ],
    notes: ['Reunión a las 3pm', 'Comprar víveres', 'Deploy v2.1'],
    status: ['Sin conexión', 'En línea', 'Sincronizando...', 'Sincronizado'],
    connect: 'Conectar Todos',
    disconnect: 'Desconectar',
    next: 'Siguiente',
    back: 'Atrás',
    reset: 'Reiniciar',
    synced: 'nuevo',
    syncMsg: 'Intercambiando vectores de estado entre peers...',
    convergedMsg: 'Todos los estados son idénticos — convergencia garantizada',
  },
};

const deviceColors = ['#00C9A7', '#F7931A', '#A259FF'];

const DeviceIcon = ({ index }: { index: number }) => {
  const props = { width: 18, height: 18, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', strokeWidth: 1.5, strokeLinecap: 'round' as const, strokeLinejoin: 'round' as const };
  if (index === 0) return <svg {...props}><rect x="5" y="2" width="14" height="20" rx="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>;
  if (index === 1) return <svg {...props}><rect x="3" y="4" width="18" height="12" rx="2"/><line x1="2" y1="20" x2="22" y2="20"/></svg>;
  return <svg {...props}><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="9" y="9" width="6" height="6"/><line x1="9" y1="2" x2="9" y2="4"/><line x1="15" y1="2" x2="15" y2="4"/><line x1="9" y1="20" x2="9" y2="22"/><line x1="15" y1="20" x2="15" y2="22"/><line x1="2" y1="9" x2="4" y2="9"/><line x1="2" y1="15" x2="4" y2="15"/><line x1="20" y1="9" x2="22" y2="9"/><line x1="20" y1="15" x2="22" y2="15"/></svg>;
};

const WifiIcon = ({ on }: { on: boolean }) => (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    {on ? (<>
      <path d="M5 12.55a11 11 0 0114 0"/><path d="M1.42 9a16 16 0 0121.16 0"/>
      <path d="M8.53 16.11a6 6 0 016.95 0"/><circle cx="12" cy="20" r="1" fill="currentColor"/>
    </>) : (<>
      <line x1="1" y1="1" x2="23" y2="23"/><path d="M16.72 11.06A10.94 10.94 0 0119 12.55"/>
      <path d="M5 12.55a10.94 10.94 0 015.17-2.39"/><path d="M10.71 5.05A16 16 0 0122.56 9"/>
      <path d="M1.42 9a15.91 15.91 0 014.7-2.88"/><path d="M8.53 16.11a6 6 0 016.95 0"/>
      <circle cx="12" cy="20" r="1" fill="currentColor"/>
    </>)}
  </svg>
);

export default function MergeFlow({ lang }: Props) {
  const [step, setStep] = useState(0);
  const [auto, setAuto] = useState(false);
  const t = i18n[lang];

  useEffect(() => {
    if (!auto || step >= 3) { if (step >= 3) setAuto(false); return; }
    const delay = step === 0 ? 1200 : step === 1 ? 1000 : 1500;
    const timer = setTimeout(() => setStep(s => s + 1), delay);
    return () => clearTimeout(timer);
  }, [step, auto]);

  const wifiOn = step >= 1;

  const toggleWifi = () => {
    if (step === 0) { setStep(1); setAuto(true); }
    else { setStep(0); setAuto(false); }
  };

  const getDeviceNotes = (di: number) =>
    step < 3
      ? [{ text: t.notes[di], isNew: false }]
      : t.notes.map((text, i) => ({ text, isNew: i !== di }));

  const getVector = (di: number) => {
    if (step < 2) { const v = [0, 0, 0]; v[di] = 1; return v; }
    return [1, 1, 1];
  };

  const statusDot = [
    { color: '#ef4444', bg: 'rgba(239,68,68,0.15)' },
    { color: '#00C9A7', bg: 'rgba(0,201,167,0.15)' },
    { color: '#F7931A', bg: 'rgba(247,147,26,0.15)' },
    { color: '#22c55e', bg: 'rgba(34,197,94,0.15)' },
  ][step];

  return (
    <>
      <style>{`
        @keyframes merge-pulse { 0%,100% { opacity:.3 } 50% { opacity:1 } }
        @keyframes data-dot { 0% { transform:translateX(-12px);opacity:0 } 40% { opacity:1 } 60% { opacity:1 } 100% { transform:translateX(12px);opacity:0 } }
        @keyframes glow { 0%,100% { border-color:rgba(34,197,94,.3) } 50% { border-color:rgba(34,197,94,.7) } }
        .merge-pulse { animation:merge-pulse 1.5s ease-in-out infinite }
        .data-dot { animation:data-dot 1.8s ease-in-out infinite }
        .data-dot:nth-child(2) { animation-delay:.25s }
        .data-dot:nth-child(3) { animation-delay:.5s }
        .glow-card { animation:glow 2s ease-in-out infinite }
      `}</style>

      <div className="w-full max-w-4xl mx-auto">
        {/* ===== STEPPER ===== */}
        <div className="flex items-center justify-center gap-0.5 sm:gap-1 mb-6 flex-wrap">
          {t.steps.map((s, i) => (
            <div key={i} className="flex items-center">
              <button
                onClick={() => { setStep(i); setAuto(false); }}
                className="flex items-center gap-1 sm:gap-1.5 px-2 sm:px-3 py-1.5 rounded-full text-xs font-medium border transition-all"
                style={{
                  background: i === step ? 'rgba(0,201,167,0.15)' : i < step ? 'rgba(0,201,167,0.05)' : 'transparent',
                  borderColor: i === step ? 'rgba(0,201,167,0.3)' : 'transparent',
                  color: i <= step ? '#00C9A7' : '#6b7280',
                }}
              >
                <span
                  className="w-5 h-5 rounded-full flex items-center justify-center text-[10px] font-bold shrink-0"
                  style={{
                    background: i === step ? '#00C9A7' : i < step ? 'rgba(0,201,167,0.35)' : '#374151',
                    color: i <= step ? '#0D1117' : '#6b7280',
                  }}
                >{i < step ? '\u2713' : i + 1}</span>
                <span className="hidden sm:inline">{s.label}</span>
              </button>
              {i < 3 && <div className="w-3 sm:w-8 h-px mx-0.5" style={{ background: i < step ? 'rgba(0,201,167,0.4)' : '#1f2937' }} />}
            </div>
          ))}
        </div>

        {/* ===== DESCRIPTION ===== */}
        <p className="text-center text-sm mb-8 min-h-[40px] transition-all" style={{ color: '#8b949e' }}>
          {t.steps[step].desc}
        </p>

        {/* ===== DEVICE CARDS ===== */}
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-2">
          {t.devices.map((dev, di) => {
            const notes = getDeviceNotes(di);
            const vec = getVector(di);
            return (
              <div
                key={di}
                className={`rounded-xl border overflow-hidden transition-all duration-500 ${step === 3 ? 'glow-card' : ''}`}
                style={{
                  background: step === 3 ? 'rgba(34,197,94,0.04)' : '#161b22',
                  borderColor: step === 3 ? 'rgba(34,197,94,0.4)' : step === 2 ? 'rgba(247,147,26,0.3)' : '#30363d',
                }}
              >
                {/* Header */}
                <div className="flex items-center gap-2 px-4 py-2.5 border-b" style={{ borderColor: '#21262d' }}>
                  <span style={{ color: deviceColors[di] }}><DeviceIcon index={di} /></span>
                  <div className="flex flex-col leading-tight">
                    <span className="font-semibold text-xs" style={{ color: '#e6edf3' }}>{dev.name}</span>
                    <span className="text-[10px]" style={{ color: '#484f58' }}>{dev.owner}</span>
                  </div>
                  <div
                    className="ml-auto flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-medium transition-all duration-300"
                    style={{ background: statusDot.bg, color: statusDot.color }}
                  >
                    <span
                      className={`w-1.5 h-1.5 rounded-full shrink-0 ${step === 2 ? 'merge-pulse' : ''}`}
                      style={{ background: statusDot.color }}
                    />
                    {t.status[step]}
                  </div>
                </div>

                {/* Screen */}
                <div className="p-4" style={{ minHeight: 130 }}>
                  <div className="text-[10px] uppercase tracking-wider mb-3 font-semibold flex items-center gap-1.5" style={{ color: '#484f58' }}>
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
                    Notes
                  </div>
                  <div className="space-y-2">
                    {notes.map((note, ni) => (
                      <div
                        key={ni}
                        className="flex items-center gap-2 text-xs transition-all duration-500"
                        style={{
                          color: note.isNew ? '#00C9A7' : '#e6edf3',
                          opacity: note.isNew && step < 3 ? 0 : 1,
                          transform: note.isNew && step < 3 ? 'translateY(-8px)' : 'translateY(0)',
                        }}
                      >
                        <span className="w-1.5 h-1.5 rounded-full shrink-0" style={{ background: note.isNew ? '#00C9A7' : '#484f58' }} />
                        <span className="flex-1">{note.text}</span>
                        {note.isNew && step >= 3 && (
                          <span className="text-[9px] px-1.5 py-0.5 rounded-full font-medium shrink-0" style={{ background: 'rgba(0,201,167,0.12)', color: '#00C9A7' }}>
                            {t.synced}
                          </span>
                        )}
                      </div>
                    ))}
                  </div>
                </div>

                {/* State vector */}
                <div className="px-4 py-2 border-t" style={{ borderColor: '#21262d', background: 'rgba(13,17,23,0.5)' }}>
                  <div
                    className="font-mono text-[10px] transition-all duration-500"
                    style={{ color: step >= 2 ? '#00C9A7' : '#484f58' }}
                  >
                    {'{'}<span style={{ color: vec[0] ? '#00C9A7' : '#484f58' }}>A:{vec[0]}</span>,{' '}
                    <span style={{ color: vec[1] ? '#F7931A' : '#484f58' }}>B:{vec[1]}</span>,{' '}
                    <span style={{ color: vec[2] ? '#A259FF' : '#484f58' }}>C:{vec[2]}</span>{'}'}
                  </div>
                </div>
              </div>
            );
          })}
        </div>

        {/* ===== SYNC STATUS ===== */}
        <div className="h-10 flex items-center justify-center">
          {step === 2 && (
            <div className="flex items-center gap-3">
              <div className="flex gap-1">
                {[0, 1, 2].map(i => <div key={i} className="data-dot w-1.5 h-1.5 rounded-full" style={{ background: '#F7931A' }} />)}
              </div>
              <span className="text-xs font-medium merge-pulse" style={{ color: '#F7931A' }}>
                {t.syncMsg}
              </span>
              <div className="flex gap-1">
                {[0, 1, 2].map(i => <div key={i} className="data-dot w-1.5 h-1.5 rounded-full" style={{ background: '#F7931A' }} />)}
              </div>
            </div>
          )}
          {step === 3 && (
            <div className="flex items-center gap-2 text-xs font-medium" style={{ color: '#22c55e' }}>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M20 6L9 17l-5-5"/></svg>
              {t.convergedMsg}
            </div>
          )}
        </div>

        {/* ===== CONTROLS ===== */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-4 mt-4">
          {/* WiFi toggle */}
          <button
            onClick={toggleWifi}
            className="flex items-center gap-3 px-5 py-2.5 rounded-xl text-sm font-semibold border transition-all duration-300 hover:scale-[1.02] active:scale-[0.98]"
            style={{
              background: wifiOn ? 'rgba(0,201,167,0.1)' : 'rgba(239,68,68,0.08)',
              borderColor: wifiOn ? 'rgba(0,201,167,0.3)' : 'rgba(239,68,68,0.25)',
              color: wifiOn ? '#00C9A7' : '#ef4444',
            }}
          >
            <WifiIcon on={wifiOn} />
            <span>{wifiOn ? t.disconnect : t.connect}</span>
            {/* Toggle track */}
            <div
              className="w-9 h-5 rounded-full relative transition-colors duration-300"
              style={{ background: wifiOn ? 'rgba(0,201,167,0.3)' : 'rgba(239,68,68,0.2)' }}
            >
              <div
                className="w-3.5 h-3.5 rounded-full absolute top-[3px] transition-all duration-300"
                style={{
                  background: wifiOn ? '#00C9A7' : '#ef4444',
                  left: wifiOn ? '18px' : '3px',
                }}
              />
            </div>
          </button>

          {/* Step navigation */}
          <div className="flex items-center gap-2">
            <button
              onClick={() => { setStep(0); setAuto(false); }}
              className="px-3 py-1.5 text-[11px] rounded-lg border transition-colors hover:border-[#484f58]"
              style={{ borderColor: '#30363d', color: '#6b7280' }}
            >{t.reset}</button>
            <button
              onClick={() => { setStep(s => Math.max(0, s - 1)); setAuto(false); }}
              disabled={step === 0}
              className="px-3 py-1.5 text-[11px] rounded-lg border transition-colors disabled:opacity-30 disabled:cursor-not-allowed hover:border-[#484f58]"
              style={{ borderColor: '#30363d', color: '#8b949e' }}
            >&larr; {t.back}</button>
            <div className="flex gap-1.5 mx-1">
              {[0, 1, 2, 3].map(i => (
                <button
                  key={i}
                  onClick={() => { setStep(i); setAuto(false); }}
                  className="w-2 h-2 rounded-full transition-all duration-300"
                  style={{
                    background: i === step ? '#00C9A7' : '#30363d',
                    transform: i === step ? 'scale(1.4)' : 'scale(1)',
                  }}
                />
              ))}
            </div>
            <button
              onClick={() => { setStep(s => Math.min(3, s + 1)); setAuto(false); }}
              disabled={step === 3}
              className="px-3 py-1.5 text-[11px] rounded-lg font-semibold transition-all disabled:opacity-30 disabled:cursor-not-allowed hover:brightness-110"
              style={{ background: '#00C9A7', color: '#0D1117' }}
            >{t.next} &rarr;</button>
          </div>
        </div>
      </div>
    </>
  );
}
