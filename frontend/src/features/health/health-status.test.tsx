import { screen } from "@testing-library/react"
import { describe, expect, it, vi } from "vitest"

import { HealthStatus } from "@/features/health/health-status"
import { mockFetchJson, renderWithProviders } from "@/test/render"

describe("HealthStatus", () => {
  it("indique que le serveur est opérationnel quand l'API et la base répondent", async () => {
    const fetch = vi.fn(mockFetchJson(200, { status: "ok", database: "ok" }))
    vi.stubGlobal("fetch", fetch)

    renderWithProviders(<HealthStatus />)

    expect(await screen.findByText("Serveur opérationnel")).toBeInTheDocument()
    const request = fetch.mock.calls[0][0] as Request
    expect(new URL(request.url).pathname).toBe("/api/health/ready")
  })

  it("indique que le serveur est indisponible quand la base ne répond pas", async () => {
    const problem = {
      status: 503,
      title: "Service Unavailable",
      detail: "Base injoignable",
    }
    vi.stubGlobal("fetch", vi.fn(mockFetchJson(503, problem)))

    renderWithProviders(<HealthStatus />)

    expect(await screen.findByText("Serveur indisponible")).toBeInTheDocument()
  })

  it("affiche un indicateur de chargement en attendant la réponse", () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(() => new Promise(() => {}))
    )

    renderWithProviders(<HealthStatus />)

    expect(screen.getByLabelText("Vérification du serveur")).toBeInTheDocument()
  })
})
