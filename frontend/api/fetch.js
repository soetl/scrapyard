export function fetchBaseURL(request, opts) {
  const config = useRuntimeConfig();

  return $fetch(request, { baseURL: config.public.baseURL, ...opts });
}

export function useFetchBaseURL(request, opts) {
  const config = useRuntimeConfig();

  return useFetch(request, { baseURL: config.public.baseURL, ...opts });
}
