import { createMemoryHistory, RouterProvider } from "@tanstack/react-router"
import { screen } from "@testing-library/react"
import { describe, expect, it, vi } from "vitest"

import { createAppRouter } from "@/app/router"
import { mockFetchJson, renderWithProviders } from "@/test/render"

function renderAt(path: string) {
  vi.stubGlobal(
    "fetch",
    vi.fn(mockFetchJson(200, { status: "ok", database: "ok" }))
  )
  const router = createAppRouter(
    createMemoryHistory({ initialEntries: [path] })
  )
  renderWithProviders(<RouterProvider router={router} />)
}

describe("routeur de l'application", () => {
  it("affiche l'accueil avec l'état du serveur", async () => {
    renderAt("/")
    expect(
      await screen.findByRole("heading", { name: "Bienvenue sur e_school" })
    ).toBeInTheDocument()
    expect(await screen.findByText("Serveur opérationnel")).toBeInTheDocument()
  })

  it("affiche une page introuvable pour une route inconnue", async () => {
    renderAt("/n-existe-pas")
    expect(
      await screen.findByRole("heading", { name: "Page introuvable" })
    ).toBeInTheDocument()
  })

  it("garde l'en-tête de l'application sur toutes les pages", async () => {
    renderAt("/n-existe-pas")
    expect(await screen.findByRole("banner")).toHaveTextContent("e_school")
  })
})
