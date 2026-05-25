import { Component, type ErrorInfo, type ReactNode } from "react"

interface Props {
  children: ReactNode
  fallback: (error: Error, retry: () => void) => ReactNode
  onReset?: () => void
}

interface State {
  error: Error | null
}

export class ErrorBoundary extends Component<Props, State> {
  override state: State = { error: null }

  static getDerivedStateFromError(error: Error): State {
    return { error }
  }

  override componentDidCatch(error: Error, info: ErrorInfo) {
    console.error("[ErrorBoundary]", error, info.componentStack)
  }

  retry = () => {
    this.props.onReset?.()
    this.setState({ error: null })
  }

  override render() {
    if (this.state.error) {
      return this.props.fallback(this.state.error, this.retry)
    }
    return this.props.children
  }
}
