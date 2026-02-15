import { useState, useRef, useEffect, useCallback } from 'react';

interface Message {
  id: string;
  author: 'alice' | 'bob';
  text: string;
  ts: number;
}

type NetworkState = 'offline' | 'online';

const SAMPLE_MESSAGES_ALICE = [
  "Hey! How's the project going?",
  "I pushed the new feature 🚀",
  "Let's sync up tomorrow",
  "Found the bug, fixing now",
  "All tests passing ✓",
];

const SAMPLE_MESSAGES_BOB = [
  "Going great! Almost done",
  "Nice, I'll review it",
  "Sounds good, morning works",
  "Awesome, I'll pull the fix",
  "Ship it! 🎉",
];

export default function InteractiveDemo() {
  const [aliceMessages, setAliceMessages] = useState<Message[]>([]);
  const [bobMessages, setBobMessages] = useState<Message[]>([]);
  const [aliceNet, setAliceNet] = useState<NetworkState>('offline');
  const [bobNet, setBobNet] = useState<NetworkState>('offline');
  const [aliceIdx, setAliceIdx] = useState(0);
  const [bobIdx, setBobIdx] = useState(0);
  const [clock, setClock] = useState(0);
  const [syncing, setSyncing] = useState(false);
  const [justSynced, setJustSynced] = useState(false);
  const aliceRef = useRef<HTMLDivElement>(null);
  const bobRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = useCallback((ref: React.RefObject<HTMLDivElement | null>) => {
    if (ref.current) {
      ref.current.scrollTop = ref.current.scrollHeight;
    }
  }, []);

  useEffect(() => { scrollToBottom(aliceRef); }, [aliceMessages, scrollToBottom]);
  useEffect(() => { scrollToBottom(bobRef); }, [bobMessages, scrollToBottom]);

  const sendAlice = () => {
    const text = SAMPLE_MESSAGES_ALICE[aliceIdx % SAMPLE_MESSAGES_ALICE.length];
    const ts = clock + 1;
    const msg: Message = { id: `a-${ts}`, author: 'alice', text, ts };
    setAliceMessages(prev => [...prev, msg]);
    setClock(ts);
    setAliceIdx(i => i + 1);
    setJustSynced(false);
  };

  const sendBob = () => {
    const text = SAMPLE_MESSAGES_BOB[bobIdx % SAMPLE_MESSAGES_BOB.length];
    const ts = clock + 1;
    const msg: Message = { id: `b-${ts}`, author: 'bob', text, ts };
    setBobMessages(prev => [...prev, msg]);
    setClock(ts);
    setBobIdx(i => i + 1);
    setJustSynced(false);
  };

  const merge = () => {
    setSyncing(true);
    setAliceNet('online');
    setBobNet('online');
    setJustSynced(false);

    setTimeout(() => {
      const allMap = new Map<string, Message>();
      aliceMessages.forEach(m => allMap.set(m.id, m));
      bobMessages.forEach(m => allMap.set(m.id, m));
      const merged = [...allMap.values()].sort((a, b) => a.ts - b.ts);
      setAliceMessages(merged);
      setBobMessages(merged);
      setSyncing(false);
      setJustSynced(true);

      setTimeout(() => {
        setAliceNet('offline');
        setBobNet('offline');
      }, 2000);
    }, 800);
  };

  const reset = () => {
    setAliceMessages([]);
    setBobMessages([]);
    setAliceNet('offline');
    setBobNet('offline');
    setAliceIdx(0);
    setBobIdx(0);
    setClock(0);
    setSyncing(false);
    setJustSynced(false);
  };

  const aliceOnly = aliceMessages.filter(m => !bobMessages.some(b => b.id === m.id)).length;
  const bobOnly = bobMessages.filter(m => !aliceMessages.some(a => a.id === m.id)).length;
  const isConverged = justSynced && aliceMessages.length === bobMessages.length &&
    aliceMessages.every((m, i) => m.id === bobMessages[i]?.id);

  return (
    <div className="max-w-3xl mx-auto">
      {/* Devices side by side */}
      <div className="grid md:grid-cols-2 gap-4 mb-5">
        <DevicePanel
          name="Alice"
          emoji="📱"
          color="teal"
          messages={aliceMessages}
          network={aliceNet}
          scrollRef={aliceRef}
          pendingOut={aliceOnly}
          syncing={syncing}
          converged={isConverged}
        />
        <DevicePanel
          name="Bob"
          emoji="💻"
          color="orange"
          messages={bobMessages}
          network={bobNet}
          scrollRef={bobRef}
          pendingOut={bobOnly}
          syncing={syncing}
          converged={isConverged}
        />
      </div>

      {/* Actions */}
      <div className="flex justify-center gap-2 flex-wrap mb-4">
        <ActionBtn onClick={sendAlice} disabled={syncing}>
          <span className="text-teal">Alice</span> sends
        </ActionBtn>
        <ActionBtn onClick={sendBob} disabled={syncing}>
          <span className="text-orange">Bob</span> sends
        </ActionBtn>
        <ActionBtn onClick={merge} variant="merge" disabled={syncing || (aliceMessages.length === 0 && bobMessages.length === 0)}>
          {syncing ? 'Syncing...' : 'Sync / Merge'}
        </ActionBtn>
        <ActionBtn onClick={reset} variant="reset" disabled={syncing}>
          Reset
        </ActionBtn>
      </div>

      {/* Status bar */}
      <div className="text-center">
        {isConverged && (
          <div className="inline-flex items-center gap-2 bg-teal/10 border border-teal/20 rounded-lg px-4 py-2 text-teal text-xs font-semibold animate-fade-in">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M20 6L9 17l-5-5"/></svg>
            Both devices converged — same state, same order. No conflicts.
          </div>
        )}
        {!isConverged && (aliceOnly > 0 || bobOnly > 0) && (
          <p className="text-text-dim text-xs">
            {aliceOnly > 0 && <span className="text-teal">{aliceOnly} message{aliceOnly > 1 ? 's' : ''} only on Alice</span>}
            {aliceOnly > 0 && bobOnly > 0 && <span> · </span>}
            {bobOnly > 0 && <span className="text-orange">{bobOnly} message{bobOnly > 1 ? 's' : ''} only on Bob</span>}
            <span className="ml-2">— press Sync to merge</span>
          </p>
        )}
      </div>
    </div>
  );
}

function DevicePanel({ name, emoji, color, messages, network, scrollRef, pendingOut, syncing, converged }: {
  name: string;
  emoji: string;
  color: 'teal' | 'orange';
  messages: Message[];
  network: NetworkState;
  scrollRef: React.RefObject<HTMLDivElement | null>;
  pendingOut: number;
  syncing: boolean;
  converged: boolean;
}) {
  const borderColor = color === 'teal' ? 'border-teal/30' : 'border-orange/30';
  const borderConverged = converged ? (color === 'teal' ? 'border-teal' : 'border-orange') : borderColor;
  const textColor = color === 'teal' ? 'text-teal' : 'text-orange';
  const bgDot = color === 'teal' ? 'bg-teal' : 'bg-orange';

  return (
    <div className={`bg-bg-card border-2 rounded-xl overflow-hidden transition-all duration-500 ${borderConverged} ${converged ? 'shadow-[0_0_20px_rgba(0,201,167,0.1)]' : ''}`}>
      {/* Header */}
      <div className="flex items-center gap-2 px-4 py-2.5 border-b border-border">
        <span className="text-base">{emoji}</span>
        <span className={`font-bold text-sm ${textColor}`}>{name}</span>
        <span className="ml-auto flex items-center gap-1.5 text-[10px] font-mono">
          <span className={`w-1.5 h-1.5 rounded-full ${network === 'online' ? 'bg-green' : 'bg-text-dim'} ${syncing ? 'animate-pulse' : ''}`}></span>
          <span className="text-text-dim">{network === 'online' ? (syncing ? 'syncing' : 'online') : 'offline'}</span>
        </span>
        {pendingOut > 0 && !converged && (
          <span className={`${bgDot}/10 ${textColor} text-[10px] font-bold px-1.5 py-0.5 rounded-full`}>
            {pendingOut} pending
          </span>
        )}
      </div>

      {/* Chat area */}
      <div ref={scrollRef} className="h-48 overflow-y-auto p-3 space-y-2 scroll-smooth" style={{ scrollbarWidth: 'thin' }}>
        {messages.length === 0 && (
          <div className="h-full flex items-center justify-center text-text-dim text-xs italic">
            No messages yet
          </div>
        )}
        {messages.map(msg => (
          <ChatBubble key={msg.id} msg={msg} side={color === 'teal' ? 'alice' : 'bob'} />
        ))}
      </div>
    </div>
  );
}

function ChatBubble({ msg, side }: { msg: Message; side: 'alice' | 'bob' }) {
  const isOwn = msg.author === side;
  return (
    <div className={`flex ${isOwn ? 'justify-end' : 'justify-start'}`}>
      <div className={`max-w-[85%] rounded-xl px-3 py-2 text-xs leading-relaxed ${
        isOwn
          ? 'bg-teal/10 text-text border border-teal/10'
          : 'bg-orange/10 text-text border border-orange/10'
      }`}>
        <div className={`text-[9px] font-bold mb-0.5 ${msg.author === 'alice' ? 'text-teal' : 'text-orange'}`}>
          {msg.author === 'alice' ? 'Alice' : 'Bob'}
        </div>
        {msg.text}
        <span className="text-text-dim text-[8px] ml-2">ts:{msg.ts}</span>
      </div>
    </div>
  );
}

function ActionBtn({ children, onClick, variant = 'default', disabled = false }: {
  children: React.ReactNode;
  onClick: () => void;
  variant?: 'default' | 'merge' | 'reset';
  disabled?: boolean;
}) {
  const cls = {
    default: 'border-border text-text hover:border-teal hover:text-teal',
    merge: 'bg-teal/10 text-teal border-teal/30 hover:bg-teal/20',
    reset: 'border-border text-text-dim hover:text-text',
  }[variant];
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={`border rounded-lg px-4 py-2 font-mono text-xs transition-all disabled:opacity-40 disabled:cursor-not-allowed ${cls}`}
    >
      {children}
    </button>
  );
}
