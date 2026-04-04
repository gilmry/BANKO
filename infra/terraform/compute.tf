# Compute instances for BANKO on OVH Cloud

data "openstack_images_image_v2" "debian" {
  provider    = openstack.ovh
  name        = "Debian 12"
  most_recent = true
}

resource "openstack_compute_instance_v2" "api" {
  provider        = openstack.ovh
  name            = "${var.project_name}-${var.environment}-api"
  image_id        = data.openstack_images_image_v2.debian.id
  flavor_name     = var.api_instance_flavor
  security_groups = [openstack_networking_secgroup_v2.web.name]

  network {
    uuid = openstack_networking_network_v2.banko_network.id
  }

  metadata = {
    project     = var.project_name
    environment = var.environment
    role        = "api"
  }
}

resource "openstack_compute_instance_v2" "frontend" {
  provider        = openstack.ovh
  name            = "${var.project_name}-${var.environment}-frontend"
  image_id        = data.openstack_images_image_v2.debian.id
  flavor_name     = var.frontend_instance_flavor
  security_groups = [openstack_networking_secgroup_v2.web.name]

  network {
    uuid = openstack_networking_network_v2.banko_network.id
  }

  metadata = {
    project     = var.project_name
    environment = var.environment
    role        = "frontend"
  }
}
