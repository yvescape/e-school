import {
  createRootRoute,
  createRoute,
  createRouter,
  type RouterHistory,
} from "@tanstack/react-router"

import { AppLayout } from "@/app/layout/app-layout"
import { NotFoundPage } from "@/app/layout/not-found-page"
import { HomePage } from "@/features/home/home-page"

const rootRoute = createRootRoute({
  component: AppLayout,
  notFoundComponent: NotFoundPage,
})

const homeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: HomePage,
})

// Chaque module métier ajoutera ses routes ici (lot 1 et suivants).
const routeTree = rootRoute.addChildren([homeRoute])

export function createAppRouter(history?: RouterHistory) {
  return createRouter({ routeTree, history })
}

declare module "@tanstack/react-router" {
  interface Register {
    router: ReturnType<typeof createAppRouter>
  }
}
