import { useState, useEffect, useCallback } from 'react';

interface Props { lang: 'en' | 'es'; }

// ===== i18n =====
const i18n = {
  en: {
    title: 'How Sync Coaching Works',
    subtitle: 'Watch CRDTs flow through the entire ecosystem — from offline edits to guaranteed convergence.',
    phases: [
      { label: 'Offline Edits', desc: 'Each device works independently, creating local CRDT operations. No network needed.' },
      { label: 'Connect to Agent', desc: 'Devices discover the Sync Agent via mDNS and establish spoke connections.' },
      { label: 'Delta Exchange', desc: 'The Agent compares version vectors and sends only the missing deltas to each spoke.' },
      { label: 'Convergence', desc: 'All replicas reach the exact same state. Mathematically guaranteed — zero conflicts.' },
    ],
    agent: 'Sync Agent',
    agentRole: 'Hub',
    devices: [
      { name: 'Phone', owner: 'Alice', data: 'counter: 5' },
      { name: 'Laptop', owner: 'Bob', data: 'counter: 3' },
      { name: 'Sensor', owner: 'Carol', data: 'counter: 7' },
    ],
    statusLabels: ['Offline', 'Discovering...', 'Syncing', 'Synced'],
    agentStatus: ['Idle', 'Accepting...', 'Coaching', 'Complete'],
    protocol: [
      'Devices edit locally (offline-first)',
      'HELLO → WELCOME handshake',
      'OFFER → REQUEST → DELTA exchange',
      'ACK — all states converged',
    ],
    vectorLabel: 'Version Vector',
    dataLabel: 'Local State',
    mergedValue: 'counter: 15',
    explain: 'Explain',
    explanations: [
      'Each device owns a CRDT slot. Increments only touch the local slot — no coordination needed. This is the offline-first promise.',
      'The Sync Agent broadcasts its presence via mDNS. SDK clients auto-discover and send a HELLO message with their version vector.',
      'The Agent compares vectors: "Alice has {A:5,B:0,C:0}, Bob has {A:0,B:3,C:0}". It sends only the missing deltas — minimal bandwidth.',
      'After applying deltas, every device computes the same merged value. CRDTs guarantee commutativity, associativity, and idempotency.',
    ],
    autoPlay: 'Auto-play',
    reset: 'Reset',
    next: 'Next',
    back: 'Back',
    step: 'Step',
  },
  es: {
    title: 'Cómo Funciona el Coaching de Sync',
    subtitle: 'Observa cómo los CRDTs fluyen por todo el ecosistema — desde ediciones offline hasta convergencia garantizada.',
    phases: [
      { label: 'Editar Offline', desc: 'Cada dispositivo trabaja de forma independiente, creando operaciones CRDT locales. Sin red.' },
      { label: 'Conectar al Agente', desc: 'Los dispositivos descubren el Sync Agent via mDNS y establecen conexiones spoke.' },
      { label: 'Intercambio de Deltas', desc: 'El Agente compara vectores de versión y envía solo los deltas faltantes a cada spoke.' },
      { label: 'Convergencia', desc: 'Todas las réplicas alcanzan exactamente el mismo estado. Garantizado matemáticamente.' },
    ],
    agent: 'Sync Agent',
    agentRole: 'Hub',
    devices: [
      { name: 'Teléfono', owner: 'Alice', data: 'counter: 5' },
      { name: 'Laptop', owner: 'Bob', data: 'counter: 3' },
      { name: 'Sensor', owner: 'Carol', data: 'counter: 7' },
    ],
    statusLabels: ['Sin conexión', 'Descubriendo...', 'Sincronizando', 'Sincronizado'],
    agentStatus: ['Inactivo', 'Aceptando...', 'Coaching', 'Completo'],
    protocol: [
      'Los dispositivos editan localmente (offline-first)',
      'HELLO → WELCOME handshake',
      'OFFER → REQUEST → DELTA intercambio',
      'ACK — todos los estados convergieron',
    ],
    vectorLabel: 'Vector de Versión',
    dataLabel: 'Estado Local',
    mergedValue: 'counter: 15',
    explain: 'Explicar',
    explanations: [
      'Cada dispositivo posee un slot CRDT. Los incrementos solo tocan el slot local — sin coordinación. Esta es la promesa offline-first.',
      'El Sync Agent anuncia su presencia via mDNS. Los clientes SDK lo descubren automáticamente y envían un HELLO con su vector de versión.',
      'El Agente compara vectores: "Alice tiene {A:5,B:0,C:0}, Bob tiene {A:0,B:3,C:0}". Envía solo los deltas faltantes — mínimo ancho de banda.',
      'Después de aplicar deltas, cada dispositivo calcula el mismo valor. Los CRDTs garantizan conmutatividad, asociatividad e idempotencia.',
    ],
    autoPlay: 'Auto-play',
    reset: 'Reiniciar',
    next: 'Siguiente',
    back: 'Atrás',
    step: 'Paso',
  },
};

const deviceColors = ['#00C9A7', '#F7931A', '#A259FF'];
const agentColor = '#3B82F6';

// ===== Icons =====
const PhoneIcon = () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="5" y="2" width="14" height="20" rx="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>;
const LaptopIcon = () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="4" width="18" height="12" rx="2"/><line x1="2" y1="20" x2="22" y2="20"/></svg>;
const SensorIcon = () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="9" y="9" width="6" height="6"/><line x1="9" y1="2" x2="9" y2="4"/><line x1="15" y1="2" x2="15" y2="4"/><line x1="9" y1="20" x2="9" y2="22"/><line x1="15" y1="20" x2="15" y2="22"/></svg>;
const AgentIcon = () => <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="3"/><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/></svg>;
const CheckIcon = () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M20 6L9 17l-5-5"/></svg>;
const InfoIcon = () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>;

const deviceIcons = [<PhoneIcon />, <LaptopIcon />, <SensorIcon />];

export default function CoachingDemo({ lang }: Props) {
  const [phase, setPhase] = useState(0);
  const [autoPlay, setAutoPlay] = useState(false);
  const [showExplain, setShowExplain] = useState(false);
  const t = i18n[lang];

  useEffect(() => {
    if (!autoPlay) return;
    if (phase >= 3) { setAutoPlay(false); return; }
    const delay = [1800, 1500, 2000, 0][phase];
    const timer = setTimeout(() => setPhase(p => p + 1), delay);
    return () => clearTimeout(timer);
  }, [phase, autoPlay]);

  const reset = useCallback(() => {
    setPhase(0);
    setAutoPlay(false);
    setShowExplain(false);
  }, []);

  // Compute device state per phase
  const getVector = (di: number) => {
    const vals = [0, 0, 0];
    if (phase === 0) { vals[di] = [5, 3, 7][di]; }
    else if (phase === 1) { vals[di] = [5, 3, 7][di]; }
    else { vals[0] = 5; vals[1] = 3; vals[2] = 7; }
    return vals;
  };

  const getData = (di: number) => {
    if (phase < 2) return t.devices[di].data;
    return t.mergedValue;
  };

  const statusDots = [
    { color: '#ef4444', bg: 'rgba(239,68,68,0.15)' },
    { color: '#F7931A', bg: 'rgba(247,147,26,0.15)' },
    { color: '#3B82F6', bg: 'rgba(59,130,246,0.15)' },
    { color: '#22c55e', bg: 'rgba(34,197,94,0.15)' },
  ];

  const agentDots = [
    { color: '#6b7280', bg: 'rgba(107,114,128,0.15)' },
    { color: '#F7931A', bg: 'rgba(247,147,26,0.15)' },
    { color: '#3B82F6', bg: 'rgba(59,130,246,0.15)' },
    { color: '#22c55e', bg: 'rgba(34,197,94,0.15)' },
  ];

  return (
    <>
      <style>{`
        @keyframes coaching-pulse { 0%,100% { opacity:.4 } 50% { opacity:1 } }
        @keyframes coaching-dot { 0% { transform:translateY(0);opacity:0 } 30% { opacity:1 } 70% { opacity:1 } 100% { transform:translateY(-30px);opacity:0 } }
        @keyframes coaching-dot-down { 0% { transform:translateY(0);opacity:0 } 30% { opacity:1 } 70% { opacity:1 } 100% { transform:translateY(30px);opacity:0 } }
        @keyframes coaching-glow { 0%,100% { box-shadow:0 0 0 0 rgba(34,197,94,0) } 50% { box-shadow:0 0 20px 4px rgba(34,197,94,0.15) } }
        @keyframes coaching-connect { 0% { stroke-dashoffset:60 } 100% { stroke-dashoffset:0 } }
        @keyframes coaching-data-flow { 0% { offset-distance:0% } 100% { offset-distance:100% } }
        @keyframes coaching-expand { 0% { transform:scaleY(0);opacity:0 } 100% { transform:scaleY(1);opacity:1 } }
        .coaching-pulse { animation:coaching-pulse 1.5s ease-in-out infinite }
        .coaching-glow { animation:coaching-glow 2s ease-in-out infinite }
        .coaching-expand { animation:coaching-expand 0.3s ease-out forwards; transform-origin:top }
      `}</style>

      <div className="w-full max-w-5xl mx-auto">
        {/* ===== PHASE STEPPER ===== */}
        <div className="flex items-center justify-center gap-0.5 sm:gap-1 mb-6 flex-wrap">
          {t.phases.map((p, i) => (
            <div key={i} className="flex items-center">
              <button
                onClick={() => { setPhase(i); setAutoPlay(false); setShowExplain(false); }}
                className="flex items-center gap-1 sm:gap-1.5 px-2 sm:px-3 py-1.5 rounded-full text-xs font-medium border transition-all"
                style={{
                  background: i === phase ? 'rgba(59,130,246,0.15)' : i < phase ? 'rgba(59,130,246,0.05)' : 'transparent',
                  borderColor: i === phase ? 'rgba(59,130,246,0.3)' : 'transparent',
                  color: i <= phase ? '#3B82F6' : '#6b7280',
                }}
              >
                <span
                  className="w-5 h-5 rounded-full flex items-center justify-center text-[10px] font-bold shrink-0"
                  style={{
                    background: i === phase ? '#3B82F6' : i < phase ? 'rgba(59,130,246,0.35)' : '#374151',
                    color: i <= phase ? '#fff' : '#6b7280',
                  }}
                >{i < phase ? '\u2713' : i + 1}</span>
                <span className="hidden sm:inline">{p.label}</span>
              </button>
              {i < 3 && <div className="w-3 sm:w-8 h-px mx-0.5" style={{ background: i < phase ? 'rgba(59,130,246,0.4)' : '#1f2937' }} />}
            </div>
          ))}
        </div>

        {/* ===== PHASE DESCRIPTION ===== */}
        <p className="text-center text-sm mb-6 min-h-[40px] transition-all" style={{ color: '#8b949e' }}>
          {t.phases[phase].desc}
        </p>

        {/* ===== PROTOCOL MESSAGE ===== */}
        <div className="flex items-center justify-center gap-2 mb-6 h-8">
          <div
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full text-[11px] font-mono font-semibold transition-all duration-500"
            style={{
              background: phase === 3 ? 'rgba(34,197,94,0.1)' : 'rgba(59,130,246,0.08)',
              border: `1px solid ${phase === 3 ? 'rgba(34,197,94,0.25)' : 'rgba(59,130,246,0.2)'}`,
              color: phase === 3 ? '#22c55e' : '#3B82F6',
            }}
          >
            {phase === 3 && <CheckIcon />}
            {phase === 2 && <span className="coaching-pulse">&#9679;</span>}
            {t.protocol[phase]}
          </div>
        </div>

        {/* ===== MAIN VISUAL: AGENT + DEVICES ===== */}
        <div className="relative mb-6">
          {/* Sync Agent (Hub) — centered top */}
          <div className="flex justify-center mb-4">
            <div
              className={`rounded-xl overflow-hidden transition-all duration-500 w-full max-w-xs ${phase === 3 ? 'coaching-glow' : ''}`}
              style={{
                background: '#161b22',
                border: `2px solid ${phase >= 1 ? (phase === 3 ? 'rgba(34,197,94,0.5)' : 'rgba(59,130,246,0.4)') : '#30363d'}`,
              }}
            >
              <div className="flex items-center gap-2 px-4 py-2.5" style={{ borderBottom: '1px solid #21262d' }}>
                <span style={{ color: agentColor }}><AgentIcon /></span>
                <div className="flex flex-col leading-tight">
                  <span className="font-bold text-xs" style={{ color: '#e6edf3' }}>{t.agent}</span>
                  <span className="text-[10px]" style={{ color: '#484f58' }}>{t.agentRole}</span>
                </div>
                <div className="ml-auto flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-medium transition-all"
                  style={{ background: agentDots[phase].bg, color: agentDots[phase].color }}>
                  <span className={`w-1.5 h-1.5 rounded-full shrink-0 ${phase === 2 ? 'coaching-pulse' : ''}`}
                    style={{ background: agentDots[phase].color }} />
                  {t.agentStatus[phase]}
                </div>
              </div>
              <div className="p-4">
                {/* Agent's aggregated vector */}
                <div className="text-[10px] uppercase tracking-wider mb-2 font-semibold" style={{ color: '#484f58' }}>
                  {t.vectorLabel}
                </div>
                <div className="font-mono text-[11px] rounded-lg px-3 py-2 transition-all duration-500"
                  style={{ background: '#0d1117', border: '1px solid #21262d' }}>
                  {'{'}
                  {['A', 'B', 'C'].map((n, i) => {
                    const val = phase < 2 ? (phase === 0 ? 0 : '?') : [5, 3, 7][i];
                    return (
                      <span key={n}>
                        {i > 0 && ', '}
                        <span style={{ color: deviceColors[i] }}>{n}:{val}</span>
                      </span>
                    );
                  })}
                  {'}'}
                  {phase >= 2 && <span style={{ color: '#484f58' }}> = </span>}
                  {phase >= 2 && <span className="font-bold" style={{ color: '#22c55e' }}>15</span>}
                </div>
                {/* Connection indicators */}
                {phase >= 1 && (
                  <div className="flex items-center justify-center gap-3 mt-3">
                    {t.devices.map((d, i) => (
                      <div key={i} className="flex items-center gap-1 text-[9px] font-medium"
                        style={{ color: phase >= 2 ? deviceColors[i] : '#484f58' }}>
                        <span className="w-1.5 h-1.5 rounded-full"
                          style={{ background: phase === 1 ? '#F7931A' : phase >= 2 ? deviceColors[i] : '#484f58' }} />
                        {d.owner}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          </div>

          {/* Connection lines (SVG) */}
          {phase >= 1 && (
            <div className="flex justify-center mb-4">
              <svg width="360" height="30" viewBox="0 0 360 30" className="max-w-full">
                {[60, 180, 300].map((x, i) => (
                  <g key={i}>
                    <line x1={x} y1="0" x2={x} y2="30"
                      stroke={phase === 3 ? 'rgba(34,197,94,0.4)' : phase === 2 ? deviceColors[i] : 'rgba(59,130,246,0.3)'}
                      strokeWidth="2"
                      strokeDasharray={phase === 2 ? '4 4' : 'none'}
                      className={phase === 2 ? 'coaching-pulse' : ''}
                    />
                    {phase === 2 && (
                      <>
                        <circle cx={x} cy="8" r="2.5" fill={deviceColors[i]} opacity="0.8">
                          <animate attributeName="cy" values="0;30" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.2}s`} />
                          <animate attributeName="opacity" values="0;1;1;0" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.2}s`} />
                        </circle>
                        <circle cx={x} cy="22" r="2.5" fill={agentColor} opacity="0.8">
                          <animate attributeName="cy" values="30;0" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.3 + 0.5}s`} />
                          <animate attributeName="opacity" values="0;1;1;0" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.3 + 0.5}s`} />
                        </circle>
                      </>
                    )}
                  </g>
                ))}
              </svg>
            </div>
          )}

          {/* Devices (Spokes) — 3 cards in row */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
            {t.devices.map((dev, di) => {
              const vec = getVector(di);
              const data = getData(di);
              const dot = statusDots[phase];
              const converged = phase === 3;
              return (
                <div
                  key={di}
                  className={`rounded-xl overflow-hidden transition-all duration-500 ${converged ? 'coaching-glow' : ''}`}
                  style={{
                    background: converged ? 'rgba(34,197,94,0.03)' : '#161b22',
                    border: `2px solid ${converged ? 'rgba(34,197,94,0.5)' : phase >= 1 ? `${deviceColors[di]}50` : '#30363d'}`,
                  }}
                >
                  {/* Device header */}
                  <div className="flex items-center gap-2 px-3 py-2" style={{ borderBottom: '1px solid #21262d' }}>
                    <span style={{ color: deviceColors[di] }}>{deviceIcons[di]}</span>
                    <div className="flex flex-col leading-tight">
                      <span className="font-semibold text-xs" style={{ color: '#e6edf3' }}>{dev.name}</span>
                      <span className="text-[10px]" style={{ color: '#484f58' }}>{dev.owner}</span>
                    </div>
                    <div className="ml-auto flex items-center gap-1 px-2 py-0.5 rounded-full text-[9px] font-medium"
                      style={{ background: dot.bg, color: dot.color }}>
                      <span className={`w-1.5 h-1.5 rounded-full ${phase === 2 ? 'coaching-pulse' : ''}`}
                        style={{ background: dot.color }} />
                      {t.statusLabels[phase]}
                    </div>
                  </div>

                  {/* Device body */}
                  <div className="p-3 space-y-2">
                    {/* Data */}
                    <div>
                      <div className="text-[9px] uppercase tracking-wider mb-1 font-semibold" style={{ color: '#484f58' }}>
                        {t.dataLabel}
                      </div>
                      <div className="font-mono text-xs rounded-lg px-2.5 py-1.5 transition-all duration-500"
                        style={{
                          background: '#0d1117',
                          border: `1px solid ${converged ? 'rgba(34,197,94,0.3)' : '#21262d'}`,
                          color: converged ? '#22c55e' : '#e6edf3',
                        }}>
                        {data}
                        {converged && <span className="ml-1 text-[9px]" style={{ color: '#22c55e' }}> &#10003;</span>}
                      </div>
                    </div>

                    {/* Version vector */}
                    <div>
                      <div className="text-[9px] uppercase tracking-wider mb-1 font-semibold" style={{ color: '#484f58' }}>
                        {t.vectorLabel}
                      </div>
                      <div className="font-mono text-[10px] rounded-lg px-2.5 py-1.5"
                        style={{ background: '#0d1117', border: '1px solid #21262d', color: '#484f58' }}>
                        {'{'}
                        {['A', 'B', 'C'].map((n, i) => (
                          <span key={n}>
                            {i > 0 && ','}
                            <span style={{ color: vec[i] > 0 ? deviceColors[i] : '#484f58' }}>{n}:{vec[i]}</span>
                          </span>
                        ))}
                        {'}'}
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* ===== EXPLAIN PANEL ===== */}
        <div className="flex justify-center mb-4">
          <button
            onClick={() => setShowExplain(s => !s)}
            className="flex items-center gap-1.5 px-4 py-2 rounded-lg text-xs font-medium border transition-all"
            style={{
              background: showExplain ? 'rgba(59,130,246,0.1)' : 'transparent',
              borderColor: showExplain ? 'rgba(59,130,246,0.3)' : '#30363d',
              color: showExplain ? '#3B82F6' : '#8b949e',
            }}
          >
            <InfoIcon />
            {t.explain} {t.step} {phase + 1}
          </button>
        </div>

        {showExplain && (
          <div className="coaching-expand rounded-xl px-5 py-4 mb-6 text-xs leading-relaxed"
            style={{ background: 'rgba(59,130,246,0.05)', border: '1px solid rgba(59,130,246,0.15)', color: '#8b949e' }}>
            <div className="flex items-start gap-2">
              <span style={{ color: '#3B82F6' }} className="mt-0.5 shrink-0"><InfoIcon /></span>
              <p>{t.explanations[phase]}</p>
            </div>
          </div>
        )}

        {/* ===== CONTROLS ===== */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
          {/* Auto-play toggle */}
          <button
            onClick={() => { if (phase >= 3) { setPhase(0); setShowExplain(false); } setAutoPlay(a => !a); }}
            className="flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-semibold border transition-all hover:scale-[1.02] active:scale-[0.98]"
            style={{
              background: autoPlay ? 'rgba(59,130,246,0.1)' : 'transparent',
              borderColor: autoPlay ? 'rgba(59,130,246,0.3)' : '#30363d',
              color: autoPlay ? '#3B82F6' : '#8b949e',
            }}
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill={autoPlay ? '#3B82F6' : 'none'} stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              {autoPlay ? <><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></> : <polygon points="5 3 19 12 5 21 5 3"/>}
            </svg>
            {t.autoPlay}
          </button>

          {/* Step navigation */}
          <div className="flex items-center gap-2">
            <button onClick={reset}
              className="px-3 py-1.5 text-[11px] rounded-lg border transition-colors hover:border-[#484f58]"
              style={{ borderColor: '#30363d', color: '#6b7280' }}>
              {t.reset}
            </button>
            <button
              onClick={() => { setPhase(p => Math.max(0, p - 1)); setAutoPlay(false); setShowExplain(false); }}
              disabled={phase === 0}
              className="px-3 py-1.5 text-[11px] rounded-lg border transition-colors disabled:opacity-30 disabled:cursor-not-allowed hover:border-[#484f58]"
              style={{ borderColor: '#30363d', color: '#8b949e' }}>
              &larr; {t.back}
            </button>
            <div className="flex gap-1.5 mx-1">
              {[0, 1, 2, 3].map(i => (
                <button key={i}
                  onClick={() => { setPhase(i); setAutoPlay(false); setShowExplain(false); }}
                  className="w-2 h-2 rounded-full transition-all duration-300"
                  style={{
                    background: i === phase ? '#3B82F6' : i < phase ? 'rgba(59,130,246,0.4)' : '#30363d',
                    transform: i === phase ? 'scale(1.4)' : 'scale(1)',
                  }}
                />
              ))}
            </div>
            <button
              onClick={() => { setPhase(p => Math.min(3, p + 1)); setAutoPlay(false); setShowExplain(false); }}
              disabled={phase === 3}
              className="px-3 py-1.5 text-[11px] rounded-lg font-semibold transition-all disabled:opacity-30 disabled:cursor-not-allowed hover:brightness-110"
              style={{ background: '#3B82F6', color: '#fff' }}>
              {t.next} &rarr;
            </button>
          </div>
        </div>

        {/* ===== CONVERGENCE BANNER ===== */}
        {phase === 3 && (
          <div className="flex justify-center mt-6">
            <div className="inline-flex items-center gap-2 rounded-lg px-5 py-3 text-xs font-semibold"
              style={{ background: 'rgba(34,197,94,0.1)', border: '1px solid rgba(34,197,94,0.2)', color: '#22c55e' }}>
              <CheckIcon />
              {lang === 'es'
                ? 'Convergido — todos los dispositivos tienen counter: 15. Cero conflictos.'
                : 'Converged — all devices have counter: 15. Zero conflicts.'}
            </div>
          </div>
        )}
      </div>
    </>
  );
}
