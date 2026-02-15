import { useState, useRef, useEffect, useCallback } from 'react';

type Tab = 'chat' | 'counter' | 'cart';

// ===== SVG Icons =====
const PhoneIcon = () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="5" y="2" width="14" height="20" rx="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>;
const LaptopIcon = () => <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="4" width="18" height="12" rx="2"/><line x1="2" y1="20" x2="22" y2="20"/></svg>;
const SyncIcon = () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>;
const CheckIcon = () => <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M20 6L9 17l-5-5"/></svg>;
const TrashIcon = () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/></svg>;
const PlusIcon = () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>;
const ResetIcon = () => <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M3 12a9 9 0 119 9"/><polyline points="3 3 3 12 12 12"/></svg>;

export default function InteractiveDemo() {
  const [tab, setTab] = useState<Tab>('chat');
  const tabs: { id: Tab; label: string; sub: string }[] = [
    { id: 'chat', label: 'Live Chat', sub: 'ORSet' },
    { id: 'counter', label: 'Counter', sub: 'GCounter' },
    { id: 'cart', label: 'Shopping List', sub: 'ORSet' },
  ];

  return (
    <div className="max-w-3xl mx-auto">
      {/* Tabs */}
      <div className="flex gap-1 p-1 bg-bg-card border border-border rounded-xl mb-6">
        {tabs.map(t => (
          <button key={t.id} onClick={() => setTab(t.id)}
            className="flex-1 flex flex-col items-center gap-0.5 py-2.5 rounded-lg text-xs font-semibold transition-all"
            style={{
              background: tab === t.id ? 'rgba(0,201,167,0.1)' : 'transparent',
              color: tab === t.id ? '#00C9A7' : '#8b949e',
              boxShadow: tab === t.id ? 'inset 0 0 0 1px rgba(0,201,167,0.2)' : 'none',
            }}>
            <span className="text-sm">{t.label}</span>
            <span className="text-[9px] font-mono opacity-60">{t.sub}</span>
          </button>
        ))}
      </div>
      {tab === 'chat' && <ChatDemo />}
      {tab === 'counter' && <CounterDemo />}
      {tab === 'cart' && <CartDemo />}
    </div>
  );
}

// ========== SHARED ==========

function ActionBtn({ children, onClick, variant = 'default', disabled = false, icon }: {
  children: React.ReactNode; onClick: () => void; variant?: 'default' | 'merge' | 'reset' | 'danger'; disabled?: boolean; icon?: React.ReactNode;
}) {
  const styles: Record<string, { bg: string; border: string; color: string; hoverBg: string }> = {
    default: { bg: 'transparent', border: '#30363d', color: '#e6edf3', hoverBg: 'rgba(0,201,167,0.05)' },
    merge: { bg: 'rgba(0,201,167,0.1)', border: 'rgba(0,201,167,0.3)', color: '#00C9A7', hoverBg: 'rgba(0,201,167,0.18)' },
    reset: { bg: 'transparent', border: '#30363d', color: '#6b7280', hoverBg: 'rgba(139,148,158,0.08)' },
    danger: { bg: 'transparent', border: '#30363d', color: '#ef4444', hoverBg: 'rgba(239,68,68,0.08)' },
  };
  const s = styles[variant];
  return (
    <button onClick={onClick} disabled={disabled}
      className="flex items-center gap-1.5 rounded-lg px-3 py-2 text-xs font-medium transition-all disabled:opacity-30 disabled:cursor-not-allowed"
      style={{ border: `1px solid ${s.border}`, background: s.bg, color: s.color }}
      onMouseEnter={e => { if (!disabled) (e.target as HTMLElement).style.background = s.hoverBg; }}
      onMouseLeave={e => (e.target as HTMLElement).style.background = s.bg}>
      {icon}{children}
    </button>
  );
}

function ConvergedBanner({ text }: { text: string }) {
  return (
    <div className="inline-flex items-center gap-2 rounded-lg px-4 py-2.5 text-xs font-semibold"
      style={{ background: 'rgba(34,197,94,0.1)', border: '1px solid rgba(34,197,94,0.2)', color: '#22c55e' }}>
      <CheckIcon />{text}
    </div>
  );
}

function DeviceFrame({ name, icon, color, badge, badgeColor, children, converged }: {
  name: string; icon: React.ReactNode; color: string; badge: string; badgeColor: string; children: React.ReactNode; converged: boolean;
}) {
  return (
    <div className="rounded-xl overflow-hidden transition-all duration-500"
      style={{
        background: '#161b22',
        border: `2px solid ${converged ? 'rgba(34,197,94,0.5)' : color === '#00C9A7' ? 'rgba(0,201,167,0.25)' : 'rgba(247,147,26,0.25)'}`,
        boxShadow: converged ? '0 0 20px rgba(34,197,94,0.08)' : 'none',
      }}>
      {/* Header bar */}
      <div className="flex items-center gap-2 px-4 py-2.5" style={{ borderBottom: '1px solid #21262d' }}>
        <span style={{ color }}>{icon}</span>
        <span className="font-bold text-xs" style={{ color }}>{name}</span>
        <div className="ml-auto flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-medium"
          style={{ background: badgeColor + '18', color: badgeColor }}>
          <span className="w-1.5 h-1.5 rounded-full" style={{ background: badgeColor }} />
          {badge}
        </div>
      </div>
      {children}
    </div>
  );
}

function InfoBox({ children }: { children: React.ReactNode }) {
  return (
    <div className="rounded-lg px-4 py-2.5 text-xs leading-relaxed text-center mb-5"
      style={{ background: 'rgba(0,201,167,0.04)', border: '1px solid rgba(0,201,167,0.1)', color: '#8b949e' }}>
      {children}
    </div>
  );
}

// ========== 1. LIVE CHAT ==========

interface Message { id: string; author: 'alice' | 'bob'; text: string; ts: number; }

const MSGS_A = ["Hey! How's the project going?", "I pushed the new feature 🚀", "Let's sync up tomorrow", "Found the bug, fixing now", "All tests passing ✓"];
const MSGS_B = ["Going great! Almost done", "Nice, I'll review it", "Sounds good, morning works", "Awesome, I'll pull the fix", "Ship it! 🎉"];

function ChatDemo() {
  const [aMsg, setAMsg] = useState<Message[]>([]);
  const [bMsg, setBMsg] = useState<Message[]>([]);
  const [aNet, setANet] = useState<'offline' | 'online'>('offline');
  const [bNet, setBNet] = useState<'offline' | 'online'>('offline');
  const [aIdx, setAIdx] = useState(0);
  const [bIdx, setBIdx] = useState(0);
  const [clock, setClock] = useState(0);
  const [syncing, setSyncing] = useState(false);
  const [synced, setSynced] = useState(false);
  const aRef = useRef<HTMLDivElement>(null);
  const bRef = useRef<HTMLDivElement>(null);
  const scroll = useCallback((r: React.RefObject<HTMLDivElement | null>) => { r.current && (r.current.scrollTop = r.current.scrollHeight); }, []);
  useEffect(() => { scroll(aRef); }, [aMsg, scroll]);
  useEffect(() => { scroll(bRef); }, [bMsg, scroll]);

  const sendA = () => { const ts = clock + 1; setAMsg(p => [...p, { id: `a-${ts}`, author: 'alice', text: MSGS_A[aIdx % MSGS_A.length], ts }]); setClock(ts); setAIdx(i => i + 1); setSynced(false); };
  const sendB = () => { const ts = clock + 1; setBMsg(p => [...p, { id: `b-${ts}`, author: 'bob', text: MSGS_B[bIdx % MSGS_B.length], ts }]); setClock(ts); setBIdx(i => i + 1); setSynced(false); };

  const merge = () => {
    setSyncing(true); setANet('online'); setBNet('online'); setSynced(false);
    setTimeout(() => {
      const m = new Map<string, Message>(); aMsg.forEach(x => m.set(x.id, x)); bMsg.forEach(x => m.set(x.id, x));
      const merged = [...m.values()].sort((a, b) => a.ts - b.ts);
      setAMsg(merged); setBMsg(merged); setSyncing(false); setSynced(true);
      setTimeout(() => { setANet('offline'); setBNet('offline'); }, 2000);
    }, 800);
  };

  const reset = () => { setAMsg([]); setBMsg([]); setANet('offline'); setBNet('offline'); setAIdx(0); setBIdx(0); setClock(0); setSyncing(false); setSynced(false); };
  const aOnly = aMsg.filter(m => !bMsg.some(b => b.id === m.id)).length;
  const bOnly = bMsg.filter(m => !aMsg.some(a => a.id === m.id)).length;
  const conv = synced && aMsg.length === bMsg.length && aMsg.every((m, i) => m.id === bMsg[i]?.id);

  const chatBadge = (net: string) => net === 'online' ? (syncing ? 'syncing' : 'online') : 'offline';
  const badgeCol = (net: string) => net === 'online' ? '#22c55e' : '#ef4444';

  return (
    <div>
      <InfoBox>
        Two devices chat <b className="text-text">offline</b>. Messages are unique by ID.
        On merge, the ORSet <b style={{ color: '#00C9A7' }}>unions all messages</b> — none are lost.
      </InfoBox>

      <div className="grid md:grid-cols-2 gap-4 mb-5">
        {/* Alice */}
        <DeviceFrame name="Alice's Phone" icon={<PhoneIcon />} color="#00C9A7"
          badge={chatBadge(aNet)} badgeColor={badgeCol(aNet)} converged={conv}>
          <div ref={aRef} className="h-52 overflow-y-auto p-3 space-y-2" style={{ scrollbarWidth: 'thin' }}>
            {aMsg.length === 0 && <EmptyChat />}
            {aMsg.map(m => <ChatBubble key={m.id} msg={m} side="alice" />)}
          </div>
          {aOnly > 0 && !conv && (
            <div className="px-3 pb-2"><span className="text-[10px] font-mono px-2 py-0.5 rounded-full" style={{ background: 'rgba(0,201,167,0.1)', color: '#00C9A7' }}>{aOnly} pending</span></div>
          )}
        </DeviceFrame>

        {/* Bob */}
        <DeviceFrame name="Bob's Laptop" icon={<LaptopIcon />} color="#F7931A"
          badge={chatBadge(bNet)} badgeColor={badgeCol(bNet)} converged={conv}>
          <div ref={bRef} className="h-52 overflow-y-auto p-3 space-y-2" style={{ scrollbarWidth: 'thin' }}>
            {bMsg.length === 0 && <EmptyChat />}
            {bMsg.map(m => <ChatBubble key={m.id} msg={m} side="bob" />)}
          </div>
          {bOnly > 0 && !conv && (
            <div className="px-3 pb-2"><span className="text-[10px] font-mono px-2 py-0.5 rounded-full" style={{ background: 'rgba(247,147,26,0.1)', color: '#F7931A' }}>{bOnly} pending</span></div>
          )}
        </DeviceFrame>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-center gap-6 flex-wrap mb-4">
        <div className="flex items-center gap-1.5">
          <span className="text-[10px] font-bold uppercase tracking-wider mr-1" style={{ color: '#00C9A7' }}>Alice</span>
          <ActionBtn onClick={sendA} disabled={syncing}>send</ActionBtn>
        </div>
        <ActionBtn onClick={merge} variant="merge" disabled={syncing || (aMsg.length === 0 && bMsg.length === 0)} icon={<SyncIcon />}>
          {syncing ? 'Syncing...' : 'Merge'}
        </ActionBtn>
        <div className="flex items-center gap-1.5">
          <span className="text-[10px] font-bold uppercase tracking-wider mr-1" style={{ color: '#F7931A' }}>Bob</span>
          <ActionBtn onClick={sendB} disabled={syncing}>send</ActionBtn>
        </div>
        <ActionBtn onClick={reset} variant="reset" icon={<ResetIcon />}>Reset</ActionBtn>
      </div>

      <div className="text-center">
        {conv && <ConvergedBanner text="Converged — same messages, same order. Zero conflicts." />}
        {!conv && (aOnly > 0 || bOnly > 0) && (
          <p className="text-xs" style={{ color: '#6b7280' }}>
            {aOnly > 0 && <span style={{ color: '#00C9A7' }}>{aOnly} only on Alice</span>}
            {aOnly > 0 && bOnly > 0 && <span className="mx-1.5">·</span>}
            {bOnly > 0 && <span style={{ color: '#F7931A' }}>{bOnly} only on Bob</span>}
            <span className="ml-2">— press Merge</span>
          </p>
        )}
      </div>
    </div>
  );
}

function EmptyChat() {
  return (
    <div className="h-full flex flex-col items-center justify-center gap-2">
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#30363d" strokeWidth="1" strokeLinecap="round" strokeLinejoin="round">
        <path d="M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2z"/>
      </svg>
      <span className="text-[11px]" style={{ color: '#484f58' }}>No messages yet — click send</span>
    </div>
  );
}

function ChatBubble({ msg, side }: { msg: Message; side: 'alice' | 'bob' }) {
  const own = msg.author === side;
  const isAlice = msg.author === 'alice';
  return (
    <div className={`flex ${own ? 'justify-end' : 'justify-start'}`}>
      <div className="max-w-[85%] rounded-xl px-3 py-2 text-xs leading-relaxed"
        style={{
          background: isAlice ? 'rgba(0,201,167,0.08)' : 'rgba(247,147,26,0.08)',
          border: `1px solid ${isAlice ? 'rgba(0,201,167,0.12)' : 'rgba(247,147,26,0.12)'}`,
        }}>
        <div className="text-[9px] font-bold mb-0.5" style={{ color: isAlice ? '#00C9A7' : '#F7931A' }}>
          {isAlice ? 'Alice' : 'Bob'}
        </div>
        <span style={{ color: '#e6edf3' }}>{msg.text}</span>
        <span className="text-[8px] ml-2" style={{ color: '#484f58' }}>ts:{msg.ts}</span>
      </div>
    </div>
  );
}

// ========== 2. COUNTER ==========

function CounterDemo() {
  const [a, setA] = useState({ a: 0, b: 0 });
  const [b, setB] = useState({ a: 0, b: 0 });
  const [flash, setFlash] = useState<'a' | 'b' | null>(null);
  const [synced, setSynced] = useState(false);

  const doFlash = (n: 'a' | 'b') => { setFlash(n); setTimeout(() => setFlash(null), 400); };
  const incA = () => { setA(s => ({ ...s, a: s.a + 1 })); doFlash('a'); setSynced(false); };
  const incB = () => { setB(s => ({ ...s, b: s.b + 1 })); doFlash('b'); setSynced(false); };
  const inc5A = () => { setA(s => ({ ...s, a: s.a + 5 })); doFlash('a'); setSynced(false); };
  const inc5B = () => { setB(s => ({ ...s, b: s.b + 5 })); doFlash('b'); setSynced(false); };

  const merge = () => {
    const ma = Math.max(a.a, b.a), mb = Math.max(a.b, b.b);
    setA({ a: ma, b: mb }); setB({ a: ma, b: mb });
    doFlash('a'); setTimeout(() => doFlash('b'), 100);
    setSynced(true);
  };

  const reset = () => { setA({ a: 0, b: 0 }); setB({ a: 0, b: 0 }); setFlash(null); setSynced(false); };
  const totalA = a.a + a.b, totalB = b.a + b.b;
  const conv = synced && totalA === totalB && a.a === b.a && a.b === b.b;
  const diverged = totalA !== totalB;

  return (
    <div>
      <InfoBox>
        GCounter: each device owns a <b style={{ color: '#00C9A7' }}>slot</b>.
        On merge, take <code style={{ color: '#00C9A7', fontFamily: 'monospace' }}>max()</code> of each slot.
        Total = sum of all slots. <b className="text-text">No increment is ever lost.</b>
      </InfoBox>

      <div className="grid md:grid-cols-2 gap-4 mb-4">
        {/* Device A */}
        <DeviceFrame name="Device A" icon={<PhoneIcon />} color="#00C9A7"
          badge={conv ? 'synced' : diverged ? 'diverged' : 'idle'} badgeColor={conv ? '#22c55e' : diverged ? '#F7931A' : '#6b7280'} converged={conv}>
          <div className="p-5 text-center">
            <div className="text-5xl font-extrabold transition-all duration-300"
              style={{ color: flash === 'a' ? '#00C9A7' : '#e6edf3', transform: flash === 'a' ? 'scale(1.1)' : 'scale(1)' }}>
              {totalA}
            </div>
            <div className="mt-3 flex items-center justify-center gap-3">
              <div className="rounded-lg px-3 py-1.5" style={{ background: 'rgba(0,201,167,0.08)', border: '1px solid rgba(0,201,167,0.15)' }}>
                <div className="text-[9px] uppercase tracking-wider mb-0.5" style={{ color: '#484f58' }}>my slot</div>
                <div className="font-mono text-sm font-bold" style={{ color: '#00C9A7' }}>{a.a}</div>
              </div>
              <div className="text-text-dim text-lg">+</div>
              <div className="rounded-lg px-3 py-1.5" style={{ background: 'rgba(247,147,26,0.06)', border: '1px solid rgba(247,147,26,0.12)' }}>
                <div className="text-[9px] uppercase tracking-wider mb-0.5" style={{ color: '#484f58' }}>remote</div>
                <div className="font-mono text-sm font-bold" style={{ color: '#F7931A' }}>{a.b}</div>
              </div>
            </div>
          </div>
        </DeviceFrame>

        {/* Device B */}
        <DeviceFrame name="Device B" icon={<LaptopIcon />} color="#F7931A"
          badge={conv ? 'synced' : diverged ? 'diverged' : 'idle'} badgeColor={conv ? '#22c55e' : diverged ? '#F7931A' : '#6b7280'} converged={conv}>
          <div className="p-5 text-center">
            <div className="text-5xl font-extrabold transition-all duration-300"
              style={{ color: flash === 'b' ? '#F7931A' : '#e6edf3', transform: flash === 'b' ? 'scale(1.1)' : 'scale(1)' }}>
              {totalB}
            </div>
            <div className="mt-3 flex items-center justify-center gap-3">
              <div className="rounded-lg px-3 py-1.5" style={{ background: 'rgba(0,201,167,0.06)', border: '1px solid rgba(0,201,167,0.12)' }}>
                <div className="text-[9px] uppercase tracking-wider mb-0.5" style={{ color: '#484f58' }}>remote</div>
                <div className="font-mono text-sm font-bold" style={{ color: '#00C9A7' }}>{b.a}</div>
              </div>
              <div className="text-text-dim text-lg">+</div>
              <div className="rounded-lg px-3 py-1.5" style={{ background: 'rgba(247,147,26,0.08)', border: '1px solid rgba(247,147,26,0.15)' }}>
                <div className="text-[9px] uppercase tracking-wider mb-0.5" style={{ color: '#484f58' }}>my slot</div>
                <div className="font-mono text-sm font-bold" style={{ color: '#F7931A' }}>{b.b}</div>
              </div>
            </div>
          </div>
        </DeviceFrame>
      </div>

      {/* State vectors */}
      <div className="grid md:grid-cols-2 gap-3 mb-5">
        <div className="rounded-lg px-3 py-2 font-mono text-[11px] text-center"
          style={{ background: '#0d1117', border: '1px solid #21262d' }}>
          <span style={{ color: '#484f58' }}>A: </span>
          <span style={{ color: '#00C9A7' }}>{`{a:${a.a}, b:${a.b}}`}</span>
          <span style={{ color: '#484f58' }}> = </span>
          <span className="font-bold" style={{ color: '#e6edf3' }}>{totalA}</span>
        </div>
        <div className="rounded-lg px-3 py-2 font-mono text-[11px] text-center"
          style={{ background: '#0d1117', border: '1px solid #21262d' }}>
          <span style={{ color: '#484f58' }}>B: </span>
          <span style={{ color: '#F7931A' }}>{`{a:${b.a}, b:${b.b}}`}</span>
          <span style={{ color: '#484f58' }}> = </span>
          <span className="font-bold" style={{ color: '#e6edf3' }}>{totalB}</span>
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-center gap-5 flex-wrap mb-4">
        <div className="flex items-center gap-1.5">
          <span className="text-[10px] font-bold uppercase tracking-wider mr-0.5" style={{ color: '#00C9A7' }}>A</span>
          <ActionBtn onClick={incA} icon={<PlusIcon />}>1</ActionBtn>
          <ActionBtn onClick={inc5A} icon={<PlusIcon />}>5</ActionBtn>
        </div>
        <ActionBtn onClick={merge} variant="merge" icon={<SyncIcon />}>merge()</ActionBtn>
        <div className="flex items-center gap-1.5">
          <span className="text-[10px] font-bold uppercase tracking-wider mr-0.5" style={{ color: '#F7931A' }}>B</span>
          <ActionBtn onClick={incB} icon={<PlusIcon />}>1</ActionBtn>
          <ActionBtn onClick={inc5B} icon={<PlusIcon />}>5</ActionBtn>
        </div>
        <ActionBtn onClick={reset} variant="reset" icon={<ResetIcon />}>Reset</ActionBtn>
      </div>

      <div className="text-center">
        {conv && <ConvergedBanner text="Converged! max() per slot — no increment is ever lost." />}
        {!conv && diverged && (
          <p className="text-xs" style={{ color: '#6b7280' }}>
            Diverged: <span style={{ color: '#00C9A7' }}>{totalA}</span> vs <span style={{ color: '#F7931A' }}>{totalB}</span> — press merge()
          </p>
        )}
      </div>
    </div>
  );
}

// ========== 3. SHOPPING LIST ==========

const ITEMS = ['🥛 Milk', '🥚 Eggs', '🍞 Bread', '🧈 Butter', '🧀 Cheese', '🍎 Apple', '🍚 Rice', '☕ Coffee'];

interface CartItem { tag: number; item: string; source: 'alice' | 'bob' }

function CartDemo() {
  const [aSet, setASet] = useState<CartItem[]>([]);
  const [bSet, setBSet] = useState<CartItem[]>([]);
  const [tag, setTag] = useState(1);
  const [idx, setIdx] = useState(0);
  const [synced, setSynced] = useState(false);

  const addA = () => {
    const item = ITEMS[idx % ITEMS.length];
    setASet(p => [...p, { tag, item, source: 'alice' }]);
    setTag(t => t + 1); setIdx(i => i + 1); setSynced(false);
  };
  const addB = () => {
    const item = ITEMS[idx % ITEMS.length];
    setBSet(p => [...p, { tag, item, source: 'bob' }]);
    setTag(t => t + 1); setIdx(i => i + 1); setSynced(false);
  };
  const removeLastA = () => { if (!aSet.length) return; setASet(p => p.slice(0, -1)); setSynced(false); };
  const removeLastB = () => { if (!bSet.length) return; setBSet(p => p.slice(0, -1)); setSynced(false); };

  const merge = () => {
    const byTag = new Map<number, CartItem>();
    aSet.forEach(i => byTag.set(i.tag, i));
    bSet.forEach(i => byTag.set(i.tag, i));
    const merged = [...byTag.values()].sort((a, b) => a.tag - b.tag);
    setASet([...merged]); setBSet([...merged]); setSynced(true);
  };

  const reset = () => { setASet([]); setBSet([]); setTag(1); setIdx(0); setSynced(false); };
  const conv = synced && aSet.length === bSet.length && aSet.every((v, i) => v.tag === bSet[i]?.tag);
  const aOnlyCount = aSet.filter(a => !bSet.some(b => b.tag === a.tag)).length;
  const bOnlyCount = bSet.filter(b => !aSet.some(a => a.tag === b.tag)).length;

  return (
    <div>
      <InfoBox>
        ORSet (Observed-Remove Set): add and remove freely.
        On merge, <b style={{ color: '#00C9A7' }}>add wins</b> over concurrent remove — <b className="text-text">no item is accidentally lost</b>.
      </InfoBox>

      <div className="grid md:grid-cols-2 gap-4 mb-5">
        <DeviceFrame name="Alice's Phone" icon={<PhoneIcon />} color="#00C9A7"
          badge={`${aSet.length} items`} badgeColor={conv ? '#22c55e' : '#00C9A7'} converged={conv}>
          <CartList items={aSet} ownerColor="#00C9A7" />
        </DeviceFrame>
        <DeviceFrame name="Bob's Laptop" icon={<LaptopIcon />} color="#F7931A"
          badge={`${bSet.length} items`} badgeColor={conv ? '#22c55e' : '#F7931A'} converged={conv}>
          <CartList items={bSet} ownerColor="#F7931A" />
        </DeviceFrame>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-center gap-5 flex-wrap mb-4">
        <div className="flex items-center gap-1.5">
          <span className="text-[10px] font-bold uppercase tracking-wider mr-0.5" style={{ color: '#00C9A7' }}>Alice</span>
          <ActionBtn onClick={addA} icon={<PlusIcon />}>add</ActionBtn>
          <ActionBtn onClick={removeLastA} variant="danger" icon={<TrashIcon />}>last</ActionBtn>
        </div>
        <ActionBtn onClick={merge} variant="merge" icon={<SyncIcon />} disabled={aSet.length === 0 && bSet.length === 0}>merge()</ActionBtn>
        <div className="flex items-center gap-1.5">
          <span className="text-[10px] font-bold uppercase tracking-wider mr-0.5" style={{ color: '#F7931A' }}>Bob</span>
          <ActionBtn onClick={addB} icon={<PlusIcon />}>add</ActionBtn>
          <ActionBtn onClick={removeLastB} variant="danger" icon={<TrashIcon />}>last</ActionBtn>
        </div>
        <ActionBtn onClick={reset} variant="reset" icon={<ResetIcon />}>Reset</ActionBtn>
      </div>

      <div className="text-center">
        {conv && <ConvergedBanner text="Converged! Same items on both devices. Add-wins: nothing is lost." />}
        {!conv && (aOnlyCount > 0 || bOnlyCount > 0) && (
          <p className="text-xs" style={{ color: '#6b7280' }}>
            {aOnlyCount > 0 && <span style={{ color: '#00C9A7' }}>{aOnlyCount} only on Alice</span>}
            {aOnlyCount > 0 && bOnlyCount > 0 && <span className="mx-1.5">·</span>}
            {bOnlyCount > 0 && <span style={{ color: '#F7931A' }}>{bOnlyCount} only on Bob</span>}
            <span className="ml-2">— press merge()</span>
          </p>
        )}
      </div>
    </div>
  );
}

function CartList({ items, ownerColor }: { items: CartItem[]; ownerColor: string }) {
  return (
    <div className="h-48 overflow-y-auto p-3" style={{ scrollbarWidth: 'thin' }}>
      {items.length === 0 ? (
        <div className="h-full flex flex-col items-center justify-center gap-2">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#30363d" strokeWidth="1" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="9" cy="21" r="1"/><circle cx="20" cy="21" r="1"/>
            <path d="M1 1h4l2.68 13.39a2 2 0 002 1.61h9.72a2 2 0 002-1.61L23 6H6"/>
          </svg>
          <span className="text-[11px]" style={{ color: '#484f58' }}>Empty — click add</span>
        </div>
      ) : (
        <div className="space-y-1.5">
          {items.map((ci, i) => {
            const synced = ci.source === 'alice' ? ownerColor !== '#00C9A7' : ownerColor !== '#F7931A';
            return (
              <div key={`${ci.tag}-${i}`} className="flex items-center gap-2 rounded-lg px-3 py-2 text-xs transition-all"
                style={{ background: '#0d1117', border: '1px solid #21262d' }}>
                <span className="w-1.5 h-1.5 rounded-full shrink-0"
                  style={{ background: ci.source === 'alice' ? '#00C9A7' : '#F7931A' }} />
                <span style={{ color: '#e6edf3' }}>{ci.item}</span>
                <span className="ml-auto font-mono text-[9px]" style={{ color: '#484f58' }}>#{ci.tag}</span>
                {synced && (
                  <span className="text-[8px] px-1 py-0.5 rounded font-medium"
                    style={{ background: 'rgba(0,201,167,0.1)', color: '#00C9A7' }}>synced</span>
                )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
