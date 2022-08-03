terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.24.0"
    }
  }
}

provider "aws" {
  region = "eu-west-1"
}

module "submodule-1" {
  source = "./modules/submodule-1"

  prefix = "context"
  suffix = "environment"
}
