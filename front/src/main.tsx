import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BaseErrorBoundary } from "./components/base-error-boundary";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BaseErrorBoundary>
      <App />
    </BaseErrorBoundary>
  </React.StrictMode>
);
