/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_BASE_URL_PUBLIC: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
