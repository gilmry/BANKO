# S3-compatible object storage for backups and documents

resource "openstack_objectstorage_container_v1" "backups" {
  provider       = openstack.ovh
  name           = "${var.project_name}-${var.environment}-backups"
  container_read = ".r:*"
}

resource "openstack_objectstorage_container_v1" "documents" {
  provider = openstack.ovh
  name     = "${var.project_name}-${var.environment}-documents"
}

resource "openstack_objectstorage_container_v1" "logs" {
  provider = openstack.ovh
  name     = "${var.project_name}-${var.environment}-logs"
}
