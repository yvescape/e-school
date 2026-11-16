import { Link } from "@tanstack/react-router"

import { Button } from "@/components/ui/button"

export function NotFoundPage() {
  return (
    <div className="flex flex-col items-start gap-4">
      <h1 className="text-2xl font-semibold">Page introuvable</h1>
      <p className="text-muted-foreground">
        Cette page n'existe pas ou a été déplacée.
      </p>
      <Button render={<Link to="/" />}>Retour à l'accueil</Button>
    </div>
  )
}
