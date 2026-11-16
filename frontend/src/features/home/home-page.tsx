import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { HealthStatus } from "@/features/health/health-status"

export function HomePage() {
  return (
    <Card className="max-w-xl">
      <CardHeader>
        <CardTitle>
          <h1 className="text-xl">Bienvenue sur e_school</h1>
        </CardTitle>
        <CardDescription>
          Gestion des années, maquettes, étudiants, emplois du temps, présences
          et notes.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <HealthStatus />
      </CardContent>
    </Card>
  )
}
