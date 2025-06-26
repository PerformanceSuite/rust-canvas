"use client"

import { useState, useRef, useEffect } from "react"
import { Power, Play, Pause, SkipBack, SkipForward, Volume2, Mic, Settings, Zap } from "lucide-react"

export default function Component() {
  const [isPlaying, setIsPlaying] = useState(false)
  const [volume, setVolume] = useState(75)
  const [bass, setBass] = useState(50)
  const [treble, setTreble] = useState(50)
  const [gain, setGain] = useState(30)
  const [toggles, setToggles] = useState({
    power: true,
    reverb: false,
    echo: true,
    compressor: false,
    limiter: true,
    eq: false,
  })
  
  // Initialize channel values with fixed values to avoid hydration issues
  const [channelValues] = useState([
    { value: 75, isOn: true },
    { value: 60, isOn: false },
    { value: 85, isOn: true },
    { value: 45, isOn: false },
    { value: 90, isOn: true },
    { value: 30, isOn: true },
    { value: 65, isOn: false },
    { value: 50, isOn: true },
  ])

  const handleToggle = (key: string) => {
    setToggles((prev) => ({ ...prev, [key]: !prev[key] }))
  }

  // Color mappings to avoid dynamic class generation
  const colorClasses = {
    cyan: {
      hover: "hover:border-cyan-500/50",
      bg: "bg-cyan-400",
      bgLight: "bg-cyan-500/20",
      bgFull: "bg-cyan-500",
      bgGradient: "bg-gradient-to-t from-cyan-500 to-cyan-300",
      shadow: "shadow-cyan-400/50",
      shadowMd: "shadow-cyan-500/50",
      shadowLg: "shadow-cyan-500/30",
      text: "text-cyan-400",
      border: "border-cyan-400",
      hex: "#06b6d4"
    },
    pink: {
      hover: "hover:border-pink-500/50",
      bg: "bg-pink-400",
      bgLight: "bg-pink-500/20",
      bgFull: "bg-pink-500",
      bgGradient: "bg-gradient-to-t from-pink-500 to-pink-300",
      shadow: "shadow-pink-400/50",
      shadowMd: "shadow-pink-500/50",
      shadowLg: "shadow-pink-500/30",
      text: "text-pink-400",
      border: "border-pink-400",
      hex: "#ec4899"
    },
    green: {
      hover: "hover:border-green-500/50",
      bg: "bg-green-400",
      bgLight: "bg-green-500/20",
      bgFull: "bg-green-500",
      bgGradient: "bg-gradient-to-t from-green-500 to-green-300",
      shadow: "shadow-green-400/50",
      shadowMd: "shadow-green-500/50",
      shadowLg: "shadow-green-500/30",
      text: "text-green-400",
      border: "border-green-400",
      hex: "#10b981"
    },
    yellow: {
      hover: "hover:border-yellow-500/50",
      bg: "bg-yellow-400",
      bgLight: "bg-yellow-500/20",
      bgFull: "bg-yellow-500",
      bgGradient: "bg-gradient-to-t from-yellow-500 to-yellow-300",
      shadow: "shadow-yellow-400/50",
      shadowMd: "shadow-yellow-500/50",
      shadowLg: "shadow-yellow-500/30",
      text: "text-yellow-400",
      border: "border-yellow-400",
      hex: "#f59e0b"
    },
    red: {
      hover: "hover:border-red-500/50",
      bg: "bg-red-400",
      bgLight: "bg-red-500/20",
      bgFull: "bg-red-500",
      bgGradient: "bg-gradient-to-t from-red-500 to-red-300",
      shadow: "shadow-red-400/50",
      shadowMd: "shadow-red-500/50",
      shadowLg: "shadow-red-500/30",
      text: "text-red-400",
      border: "border-red-400",
      hex: "#ef4444"
    }
  }

  const KnobControl = ({
    label,
    value,
    onChange,
    color = "cyan",
  }: {
    label: string
    value: number
    onChange: (value: number) => void
    color?: keyof typeof colorClasses
  }) => {
    const [isDragging, setIsDragging] = useState(false)
    const knobRef = useRef<HTMLDivElement>(null)
    const rotation = (value / 100) * 270 - 135
    const colors = colorClasses[color] || colorClasses.cyan

    useEffect(() => {
      if (!isDragging) return

      const handleMouseMove = (e: MouseEvent) => {
        if (!knobRef.current) return
        
        const rect = knobRef.current.getBoundingClientRect()
        const centerX = rect.left + rect.width / 2
        const centerY = rect.top + rect.height / 2
        
        const angle = Math.atan2(e.clientY - centerY, e.clientX - centerX)
        let degrees = angle * (180 / Math.PI) + 90
        
        if (degrees < 0) degrees += 360
        
        // Map to knob range (270 degrees starting from bottom left)
        if (degrees >= 135) {
          degrees = degrees > 405 ? 405 : degrees
          const normalizedAngle = (degrees - 135) / 270
          const newValue = normalizedAngle * 100
          onChange(Math.max(0, Math.min(100, newValue)))
        }
      }

      const handleMouseUp = () => {
        setIsDragging(false)
      }

      document.addEventListener('mousemove', handleMouseMove)
      document.addEventListener('mouseup', handleMouseUp)

      return () => {
        document.removeEventListener('mousemove', handleMouseMove)
        document.removeEventListener('mouseup', handleMouseUp)
      }
    }, [isDragging, onChange])

    return (
      <div className="flex flex-col items-center space-y-2">
        <div className="relative w-16 h-16" ref={knobRef}>
          <div
            className={`w-16 h-16 rounded-full border-4 border-gray-700 bg-gray-900 relative cursor-pointer ${colors.hover}`}
            style={{
              background: `conic-gradient(from 135deg at 50% 50%, #374151 0deg, ${colors.hex} ${(value / 100) * 270}deg, #374151 ${(value / 100) * 270}deg)`,
              transition: 'none',
            }}
            onMouseDown={() => setIsDragging(true)}
          >
            <div className="absolute inset-1 rounded-full bg-gray-900 flex items-center justify-center">
              <div
                className={`w-1 h-6 ${colors.bg} rounded-full shadow-lg ${colors.shadow}`}
                style={{ 
                  transform: `rotate(${rotation}deg)`,
                  transition: 'none',
                }}
              />
            </div>
          </div>
        </div>
        <span className="text-xs text-gray-400 font-mono">{label}</span>
        <span className={`text-xs ${colors.text} font-mono`}>{value.toFixed(1)}</span>
      </div>
    )
  }

  const ToggleSwitch = ({
    label,
    isOn,
    onToggle,
    color = "cyan",
  }: {
    label: string
    isOn: boolean
    onToggle: () => void
    color?: keyof typeof colorClasses
  }) => {
    const colors = colorClasses[color] || colorClasses.cyan
    
    return (
      <div className="flex flex-col items-center space-y-2">
        <button
          onClick={onToggle}
          className={`relative w-12 h-6 rounded-full transition-all duration-300 ${
            isOn ? `${colors.bgFull} shadow-lg ${colors.shadowMd}` : "bg-gray-700"
          }`}
        >
          <div
            className={`absolute top-0.5 w-5 h-5 rounded-full transition-all duration-300 ${
              isOn ? `translate-x-6 bg-white shadow-lg ${colors.shadowLg}` : "translate-x-0.5 bg-gray-400"
            }`}
          />
        </button>
        <span className="text-xs text-gray-400 font-mono">{label}</span>
      </div>
    )
  }

  const PushButton = ({
    icon: Icon,
    label,
    isActive,
    onClick,
    color = "cyan",
    size = "md",
  }: {
    icon: any
    label: string
    isActive?: boolean
    onClick: () => void
    color?: keyof typeof colorClasses
    size?: "sm" | "md" | "lg"
  }) => {
    const sizeClasses = {
      sm: "w-10 h-10",
      md: "w-12 h-12",
      lg: "w-16 h-16",
    }
    const colors = colorClasses[color] || colorClasses.cyan

    return (
      <div className="flex flex-col items-center space-y-2">
        <button
          onClick={onClick}
          className={`${sizeClasses[size]} rounded-full border-2 transition-all duration-200 flex items-center justify-center font-mono ${
            isActive
              ? `${colors.border} ${colors.bgLight} shadow-lg ${colors.shadowLg} ${colors.text}`
              : "border-gray-600 bg-gray-800 text-gray-400 hover:border-gray-500"
          }`}
        >
          <Icon size={size === "sm" ? 16 : size === "md" ? 20 : 24} />
        </button>
        <span className="text-xs text-gray-400 font-mono">{label}</span>
      </div>
    )
  }

  const VUMeter = ({ level, color = "green" }: { level: number; color?: keyof typeof colorClasses }) => {
    const colors = colorClasses[color] || colorClasses.green
    
    return (
      <div className="flex flex-col items-center space-y-2">
        <div className="w-4 h-32 bg-gray-800 rounded-full border border-gray-600 overflow-hidden">
          <div
            className={`w-full ${colors.bgGradient} rounded-full transition-all duration-150 shadow-lg ${colors.shadowLg}`}
            style={{ height: `${level}%`, marginTop: `${100 - level}%` }}
          />
        </div>
        <span className="text-xs text-gray-400 font-mono">VU</span>
      </div>
    )
  }

  const Slider = ({
    label,
    value,
    onChange,
    color = "cyan",
    vertical = false,
  }: {
    label: string
    value: number
    onChange: (value: number) => void
    color?: keyof typeof colorClasses
    vertical?: boolean
  }) => {
    const colors = colorClasses[color] || colorClasses.cyan
    
    return (
      <div className={`flex ${vertical ? "flex-col" : "flex-row"} items-center ${vertical ? "space-y-4" : "space-x-4"}`}>
        <span className="text-xs text-gray-400 font-mono min-w-[3rem]">{label}</span>
        <div className={`relative ${vertical ? "h-24 w-2" : "w-24 h-2"} bg-gray-700 rounded-full`}>
          <div
            className={`absolute ${vertical ? "bottom-0 w-full" : "left-0 h-full"} ${colors.bgFull} rounded-full shadow-lg ${colors.shadowLg}`}
            style={vertical ? { height: `${value}%`, transition: 'none' } : { width: `${value}%`, transition: 'none' }}
          />
          <input
            type="range"
            min="0"
            max="1000"
            step="1"
            value={value * 10}
            onChange={(e) => onChange(Number(e.target.value) / 10)}
            className={`absolute inset-0 ${vertical ? "w-full h-full" : "w-full h-full"} opacity-0 cursor-pointer`}
            style={vertical ? { writingMode: "bt-lr" as any, WebkitAppearance: "slider-vertical" as any } : {}}
          />
        </div>
        <span className={`text-xs ${colors.text} font-mono min-w-[2rem]`}>{value}</span>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-black p-8">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold text-white mb-2 font-mono">
            AUDIO <span className="text-cyan-400">CONTROL</span> MATRIX
          </h1>
          <div className="w-32 h-1 bg-gradient-to-r from-cyan-500 to-pink-500 mx-auto rounded-full shadow-lg shadow-cyan-500/30" />
        </div>

        {/* Main Control Panel */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-8">
          {/* Left Panel - Master Controls */}
          <div className="bg-gray-900 rounded-2xl p-6 border border-gray-700 shadow-2xl">
            <h2 className="text-xl font-bold text-cyan-400 mb-6 font-mono">MASTER CONTROL</h2>

            <div className="grid grid-cols-2 gap-6 mb-8">
              <KnobControl label="VOLUME" value={volume} onChange={setVolume} color="cyan" />
              <KnobControl label="GAIN" value={gain} onChange={setGain} color="pink" />
            </div>

            <div className="grid grid-cols-3 gap-4 mb-6">
              <PushButton
                icon={Power}
                label="POWER"
                isActive={toggles.power}
                onClick={() => handleToggle("power")}
                color="green"
              />
              <PushButton
                icon={isPlaying ? Pause : Play}
                label={isPlaying ? "PAUSE" : "PLAY"}
                isActive={isPlaying}
                onClick={() => setIsPlaying(!isPlaying)}
                color="cyan"
                size="lg"
              />
              <PushButton icon={Settings} label="CONFIG" isActive={false} onClick={() => {}} color="yellow" />
            </div>

            <div className="grid grid-cols-4 gap-3">
              <PushButton icon={SkipBack} label="PREV" onClick={() => {}} size="sm" />
              <PushButton icon={Volume2} label="MUTE" onClick={() => {}} size="sm" />
              <PushButton icon={Mic} label="MIC" onClick={() => {}} size="sm" />
              <PushButton icon={SkipForward} label="NEXT" onClick={() => {}} size="sm" />
            </div>
          </div>

          {/* Center Panel - EQ & Effects */}
          <div className="bg-gray-900 rounded-2xl p-6 border border-gray-700 shadow-2xl">
            <h2 className="text-xl font-bold text-pink-400 mb-6 font-mono">EQ & EFFECTS</h2>

            <div className="grid grid-cols-2 gap-6 mb-8">
              <KnobControl label="BASS" value={bass} onChange={setBass} color="green" />
              <KnobControl label="TREBLE" value={treble} onChange={setTreble} color="yellow" />
            </div>

            <div className="space-y-4 mb-6">
              <Slider label="LOW" value={60} onChange={() => {}} color="green" />
              <Slider label="MID" value={45} onChange={() => {}} color="yellow" />
              <Slider label="HIGH" value={70} onChange={() => {}} color="pink" />
            </div>

            <div className="grid grid-cols-3 gap-4">
              <ToggleSwitch label="REVERB" isOn={toggles.reverb} onToggle={() => handleToggle("reverb")} color="cyan" />
              <ToggleSwitch label="ECHO" isOn={toggles.echo} onToggle={() => handleToggle("echo")} color="pink" />
              <ToggleSwitch label="EQ" isOn={toggles.eq} onToggle={() => handleToggle("eq")} color="green" />
            </div>
          </div>

          {/* Right Panel - Monitoring */}
          <div className="bg-gray-900 rounded-2xl p-6 border border-gray-700 shadow-2xl">
            <h2 className="text-xl font-bold text-green-400 mb-6 font-mono">MONITORING</h2>

            <div className="flex justify-center space-x-8 mb-8">
              <VUMeter level={75} color="green" />
              <VUMeter level={60} color="yellow" />
              <VUMeter level={85} color="pink" />
            </div>

            <div className="space-y-6 mb-6">
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-400 font-mono">INPUT</span>
                <div className="flex space-x-1">
                  {[...Array(8)].map((_, i) => (
                    <div
                      key={i}
                      className={`w-2 h-4 rounded-sm ${
                        i < 5
                          ? "bg-green-500 shadow-sm shadow-green-500/50"
                          : i < 7
                            ? "bg-yellow-500 shadow-sm shadow-yellow-500/50"
                            : "bg-red-500 shadow-sm shadow-red-500/50"
                      }`}
                    />
                  ))}
                </div>
              </div>

              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-400 font-mono">OUTPUT</span>
                <div className="flex space-x-1">
                  {[...Array(8)].map((_, i) => (
                    <div
                      key={i}
                      className={`w-2 h-4 rounded-sm ${
                        i < 6 ? "bg-cyan-500 shadow-sm shadow-cyan-500/50" : "bg-gray-600"
                      }`}
                    />
                  ))}
                </div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <ToggleSwitch
                label="COMP"
                isOn={toggles.compressor}
                onToggle={() => handleToggle("compressor")}
                color="yellow"
              />
              <ToggleSwitch label="LIMIT" isOn={toggles.limiter} onToggle={() => handleToggle("limiter")} color="red" />
            </div>
          </div>
        </div>

        {/* Bottom Panel - Advanced Controls */}
        <div className="bg-gray-900 rounded-2xl p-6 border border-gray-700 shadow-2xl">
          <h2 className="text-xl font-bold text-yellow-400 mb-6 font-mono flex items-center">
            <Zap className="mr-2" size={20} />
            ADVANCED MATRIX
          </h2>

          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-6">
            {channelValues.map((channel, i) => (
              <div key={i} className="flex flex-col items-center space-y-3">
                <div className="text-xs text-gray-400 font-mono">CH {i + 1}</div>
                <Slider
                  label=""
                  value={channel.value}
                  onChange={() => {}}
                  color={(["cyan", "pink", "green", "yellow"] as const)[i % 4]}
                  vertical
                />
                <ToggleSwitch
                  label=""
                  isOn={channel.isOn}
                  onToggle={() => {}}
                  color={(["cyan", "pink", "green", "yellow"] as const)[i % 4]}
                />
              </div>
            ))}
          </div>
        </div>

        {/* Status Bar */}
        <div className="mt-8 bg-gray-900 rounded-xl p-4 border border-gray-700 shadow-2xl">
          <div className="flex justify-between items-center text-sm font-mono">
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse shadow-sm shadow-green-500/50" />
                <span className="text-green-400">SYSTEM ONLINE</span>
              </div>
              <div className="text-gray-400">|</div>
              <div className="text-cyan-400">48kHz / 24-bit</div>
              <div className="text-gray-400">|</div>
              <div className="text-pink-400">LATENCY: 2.3ms</div>
            </div>
            <div className="flex items-center space-x-4">
              <div className="text-yellow-400">CPU: 23%</div>
              <div className="text-gray-400">|</div>
              <div className="text-green-400">RAM: 1.2GB</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}