import { Component, type ErrorInfo, type ReactNode } from "react"

interface Props {
  children: ReactNode
}

interface State {
  hasError: boolean
  error?: Error
}

/**
 * Top-level error boundary. Catches React render-time crashes and shows a
 * terminal-styled fallback instead of a white screen. Reset is a full reload —
 * the onboarding app has no client-side state worth preserving across crashes.
 */
export class ErrorBoundary extends Component<Props, State> {
  override state: State = { hasError: false }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error }
  }

  override componentDidCatch(error: Error, info: ErrorInfo) {
    // In a future iteration this is where Sentry / OpenTelemetry would go.
    console.error("[onboarding] ErrorBoundary caught:", error, info)
  }

  private handleReload = () => {
    window.location.reload()
  }

  override render() {
    if (!this.state.hasError) return this.props.children

    return (
      <div className="flex min-h-dvh items-center justify-center bg-(--bg) p-6">
        <div className="w-full max-w-md rounded-sm border border-(--destructive)/40 bg-(--surface) p-6 font-mono">
          <p className="mb-2 text-xs text-(--muted)">
            <span className="text-(--destructive)">$</span> exit 1
          </p>
          <h1 className="mb-2 text-lg font-semibold text-(--fg)">
            Something went wrong
          </h1>
          <p className="mb-4 text-xs text-(--muted)">
            The onboarding app crashed unexpectedly. Reloading usually fixes it.
            If it keeps happening, please open an issue.
          </p>

          {this.state.error?.message && (
            <pre className="mb-4 overflow-x-auto rounded border border-(--border) bg-(--bg) p-3 text-xs text-(--destructive)">
              {this.state.error.message}
            </pre>
          )}

          <button
            type="button"
            onClick={this.handleReload}
            className="w-full rounded-sm border border-(--accent) bg-(--accent) px-4 py-2 text-xs font-medium text-(--accent-fg) hover:opacity-85"
          >
            Reload page
          </button>
        </div>
      </div>
    )
  }
}
