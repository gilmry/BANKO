variable "ovh_endpoint" {
  description = "OVH API endpoint"
  type        = string
  default     = "ovh-eu"
}

variable "ovh_application_key" {
  description = "OVH API application key"
  type        = string
  sensitive   = true
}

variable "ovh_application_secret" {
  description = "OVH API application secret"
  type        = string
  sensitive   = true
}

variable "ovh_consumer_key" {
  description = "OVH API consumer key"
  type        = string
  sensitive   = true
}

variable "region" {
  description = "OVH Cloud region"
  type        = string
  default     = "GRA7"
}

variable "project_name" {
  description = "Project name for resource naming"
  type        = string
  default     = "banko"
}

variable "environment" {
  description = "Environment (dev, staging, prod)"
  type        = string
  default     = "prod"
}

variable "db_password" {
  description = "PostgreSQL database password"
  type        = string
  sensitive   = true
}

variable "api_instance_flavor" {
  description = "Flavor for API instance"
  type        = string
  default     = "b2-15"
}

variable "frontend_instance_flavor" {
  description = "Flavor for frontend instance"
  type        = string
  default     = "b2-7"
}
