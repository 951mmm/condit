import { PropsWithChildren } from "react";
import { ErrorBoundary, FallbackProps } from "react-error-boundary";

const ErrorFallback = ({error}: FallbackProps) => {
  return (
    <div role="alert">
      <p>Someting error rendering!</p>
      <pre>{error.message}</pre>
    </div>
  )
}
export const BaseErrorBoundary: React.FC<PropsWithChildren> = (props) => {
  return (
    <ErrorBoundary FallbackComponent={ErrorFallback}>
      {props.children}
    </ErrorBoundary>
  )
}