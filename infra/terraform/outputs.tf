output "api_instance_ip" {
  description = "Public IP of the API instance"
  value       = "TBD - will be populated when instances are created"
}

output "frontend_instance_ip" {
  description = "Public IP of the frontend instance"
  value       = "TBD - will be populated when instances are created"
}

output "database_endpoint" {
  description = "PostgreSQL connection endpoint"
  value       = "TBD - will be populated when database is created"
  sensitive   = true
}

output "s3_endpoint" {
  description = "S3-compatible storage endpoint"
  value       = "https://s3.${var.region}.cloud.ovh.net"
}
