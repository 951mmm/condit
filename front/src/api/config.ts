import axios, { AxiosRequestConfig } from "axios";

interface fetcherProps {
  method: "get" | "post" | "put" | "delete",
  url: string,
  body?: {},
  signal?: AbortSignal,
}

async function fetcher({ method, url, body, signal }: fetcherProps) {
  const jwtToken = localStorage.getItem("jwtToken");
  const config = {
    baseURL: import.meta.env.VITE_BASE_URL_PRIVATE,
    headers: {
      Authorization: !!jwtToken ? `Token ${jwtToken}` : "",
    },
    signal,
  } as AxiosRequestConfig;


  switch (method) {
    case "get": return (await axios.get(url, config)).data;
    case "post": return (await axios.post(url, body, config)).data;
    case "put": return (await axios.put(url, body, config)).data;
    case "delete": return (await axios.delete(url, config)).data;
    default: throw new Error("http method wrong");

  }

}

export function GET(url: string, signal?: AbortSignal) {
  return fetcher({ method: "get", url, signal });
}

export function POST(url: string, body?: {}) {
  return fetcher({ method: "post", url, body });
}

export function PUT(url: string, body?: {}) {
  return fetcher({ method: "put", url, body });
}

export function DELETE(url: string) {
  return fetcher({ method: "delete", url });
}