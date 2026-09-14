import { nextTick, onBeforeUnmount, onMounted, type Ref, watch } from "vue"
import { FitAddon } from "@xterm/addon-fit"
import { Terminal, type ITheme } from "@xterm/xterm"

interface UseTerminalOptions {
  container: Ref<HTMLDivElement | null>
  isDark: Ref<boolean>
  sessionState: Ref<"disconnected" | "connecting" | "connected">
  hostLabel: Ref<string>
  prompt: Ref<string>
}

function terminalTheme(isDark: boolean): ITheme {
  return isDark
    ? {
      background: "#1f1f1f",
      foreground: "#f3f1ed",
      cursor: "#a990ff",
      selectionBackground: "#3a3157",
      black: "#1f1f1f",
      red: "#ff7b72",
      green: "#7ee787",
      yellow: "#ffbd61",
      blue: "#8ab4ff",
      magenta: "#a990ff",
      cyan: "#79c0ff",
      white: "#f3f1ed",
    }
    : {
      background: "#fffaf4",
      foreground: "#211f1d",
      cursor: "#6f54dc",
      selectionBackground: "#ddd4ff",
      black: "#211f1d",
      red: "#b42318",
      green: "#067647",
      yellow: "#b54708",
      blue: "#175cd3",
      magenta: "#6f54dc",
      cyan: "#0e7090",
      white: "#fffaf4",
    }
}

export function useTerminal(options: UseTerminalOptions) {
  let terminal: Terminal | null = null
  let fitAddon: FitAddon | null = null
  let resizeObserver: ResizeObserver | null = null
  let dataListener: { dispose: () => void } | null = null

  function fit() {
    fitAddon?.fit()
  }

  function writePrompt() {
    terminal?.write(`\r\n${options.prompt.value}`)
  }

  onMounted(async () => {
    if (!options.container.value) return

    terminal = new Terminal({
      cursorBlink: true,
      fontFamily: 'JetBrains Mono, "SFMono-Regular", Consolas, monospace',
      fontSize: 14,
      lineHeight: 1.25,
      theme: terminalTheme(options.isDark.value),
    })

    fitAddon = new FitAddon()
    terminal.loadAddon(fitAddon)
    terminal.open(options.container.value)

    await nextTick()
    fit()

    terminal.writeln("\x1b[1;35mTermyx\x1b[0m")
    terminal.writeln(`Mock session: ${options.hostLabel.value}`)
    terminal.writeln("Press Connect to simulate opening the session.")
    terminal.write(options.prompt.value)

    dataListener = terminal.onData((data) => {
      if (data === "\r") {
        writePrompt()
        return
      }

      if (data === "\u007F") {
        terminal?.write("\b \b")
        return
      }

      terminal?.write(data)
    })

    resizeObserver = new ResizeObserver(fit)
    resizeObserver.observe(options.container.value)
    window.addEventListener("resize", fit)
  })

  watch(options.isDark, (isDark) => {
    if (!terminal) return

    terminal.options.theme = terminalTheme(isDark)
  })

  watch(options.sessionState, (sessionState) => {
    if (!terminal) return

    terminal.write("\r\n")

    if (sessionState === "connecting") {
      terminal.writeln("Connecting to mock SSH session...")
      return
    }

    if (sessionState === "connected") {
      terminal.writeln("Mock SSH session connected.")
      terminal.write(options.prompt.value)
      return
    }

    terminal.writeln("Mock SSH session disconnected.")
  })

  onBeforeUnmount(() => {
    window.removeEventListener("resize", fit)
    resizeObserver?.disconnect()
    dataListener?.dispose()
    terminal?.dispose()
  })

  return {
    fit,
  }
}
