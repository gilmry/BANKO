# VPC / Network configuration for BANKO on OVH Cloud

resource "openstack_networking_network_v2" "banko_network" {
  provider       = openstack.ovh
  name           = "${var.project_name}-${var.environment}-network"
  admin_state_up = true
}

resource "openstack_networking_subnet_v2" "banko_subnet" {
  provider   = openstack.ovh
  name       = "${var.project_name}-${var.environment}-subnet"
  network_id = openstack_networking_network_v2.banko_network.id
  cidr       = "10.0.0.0/24"
  ip_version = 4

  dns_nameservers = ["213.186.33.99", "1.1.1.1"]
}

resource "openstack_networking_secgroup_v2" "web" {
  provider    = openstack.ovh
  name        = "${var.project_name}-${var.environment}-web-sg"
  description = "Security group for web traffic"
}

resource "openstack_networking_secgroup_rule_v2" "http" {
  provider          = openstack.ovh
  direction         = "ingress"
  ethertype         = "IPv4"
  protocol          = "tcp"
  port_range_min    = 80
  port_range_max    = 80
  remote_ip_prefix  = "0.0.0.0/0"
  security_group_id = openstack_networking_secgroup_v2.web.id
}

resource "openstack_networking_secgroup_rule_v2" "https" {
  provider          = openstack.ovh
  direction         = "ingress"
  ethertype         = "IPv4"
  protocol          = "tcp"
  port_range_min    = 443
  port_range_max    = 443
  remote_ip_prefix  = "0.0.0.0/0"
  security_group_id = openstack_networking_secgroup_v2.web.id
}

resource "openstack_networking_secgroup_v2" "db" {
  provider    = openstack.ovh
  name        = "${var.project_name}-${var.environment}-db-sg"
  description = "Security group for database access"
}

resource "openstack_networking_secgroup_rule_v2" "postgres" {
  provider          = openstack.ovh
  direction         = "ingress"
  ethertype         = "IPv4"
  protocol          = "tcp"
  port_range_min    = 5432
  port_range_max    = 5432
  remote_group_id   = openstack_networking_secgroup_v2.web.id
  security_group_id = openstack_networking_secgroup_v2.db.id
}
