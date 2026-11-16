import { Link, Outlet } from "@tanstack/react-router"

export function AppLayout() {
  return (
    <div className="flex min-h-svh flex-col bg-background">
      <header className="border-b">
        <div className="mx-auto flex h-14 w-full max-w-6xl items-center px-4">
          <Link to="/" className="font-semibold tracking-tight">
            e_school
          </Link>
        </div>
      </header>
      <main className="mx-auto flex w-full max-w-6xl flex-1 flex-col px-4 py-8">
        <Outlet />
      </main>
    </div>
  )
}
