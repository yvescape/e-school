import createClient from "openapi-fetch"

import type { paths } from "./schema"

/**
 * Client typé de l'API e_school, généré depuis l'OpenAPI du backend
 * (`npm run api:generate`). `fetch` est résolu à chaque appel pour
 * pouvoir être remplacé dans les tests.
 */
export const api = createClient<paths>({
  baseUrl: window.location.origin,
  fetch: (request) => globalThis.fetch(request),
})
