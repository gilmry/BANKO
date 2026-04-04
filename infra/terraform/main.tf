terraform {
  required_version = ">= 1.5.0"

  required_providers {
    ovh = {
      source  = "ovh/ovh"
      version = "~> 0.40"
    }
    openstack = {
      source  = "terraform-provider-openstack/openstack"
      version = "~> 1.54"
    }
  }

  backend "s3" {
    bucket   = "banko-terraform-state"
    key      = "prod/terraform.tfstate"
    region   = "gra"
    endpoint = "https://s3.gra.cloud.ovh.net"

    skip_credentials_validation = true
    skip_region_validation      = true
    skip_metadata_api_check     = true
    force_path_style            = true
  }
}

provider "ovh" {
  endpoint           = var.ovh_endpoint
  application_key    = var.ovh_application_key
  application_secret = var.ovh_application_secret
  consumer_key       = var.ovh_consumer_key
}

provider "openstack" {
  auth_url    = "https://auth.cloud.ovh.net/v3"
  domain_name = "Default"
  alias       = "ovh"
}
