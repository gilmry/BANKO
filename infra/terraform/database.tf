# Managed PostgreSQL database on OVH Cloud

resource "ovh_cloud_project_database" "banko_db" {
  service_name = var.project_name
  description  = "BANKO PostgreSQL ${var.environment}"
  engine       = "postgresql"
  version      = "16"
  plan         = "business"

  nodes {
    region = var.region
  }

  flavor = "db1-7"
}

resource "ovh_cloud_project_database_postgresql_user" "banko_user" {
  service_name = ovh_cloud_project_database.banko_db.service_name
  cluster_id   = ovh_cloud_project_database.banko_db.id
  name         = "banko"
}

resource "ovh_cloud_project_database_database" "banko_database" {
  service_name = ovh_cloud_project_database.banko_db.service_name
  engine       = "postgresql"
  cluster_id   = ovh_cloud_project_database.banko_db.id
  name         = "banko_db"
}
