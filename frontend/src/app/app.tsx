import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { RouterProvider } from "@tanstack/react-router"

import { createAppRouter } from "@/app/router"
import { ThemeProvider } from "@/components/theme-provider"

const queryClient = new QueryClient()
const router = createAppRouter()

export function App() {
  return (
    <ThemeProvider>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </ThemeProvider>
  )
}
