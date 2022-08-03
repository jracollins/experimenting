data "aws_caller_identity" "current" {}

resource "aws_iam_role" "role" {
  name = "${var.prefix}-role-${var.suffix}"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "sts:AssumeRole",
        ]
        Effect = "Allow"
        Sid    = ""
        Principal = {
          AWS = data.aws_caller_identity.current.account_id
        }
      },
    ]
  })
}

resource "aws_iam_policy" "allow_role_assumable" {
  name = "${var.prefix}-policy-${var.suffix}"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "sts:AssumeRole",
        ],
        Effect   = "Allow"
        Resource = [aws_iam_role.role.arn]
      },
    ]
  })
}

resource "aws_iam_group" "group" {
  name = "${var.prefix}-group-${var.suffix}"
}

resource "aws_iam_group_policy_attachment" "attach_to_group" {
  group      = aws_iam_group.group.name
  policy_arn = aws_iam_policy.allow_role_assumable.arn
}

resource "aws_iam_user" "user" {
  name = "${var.prefix}-user-${var.suffix}"
}

resource "aws_iam_user_group_membership" "add_user_to_group" {
  user = aws_iam_user.user.name

  groups = [
    aws_iam_group.group.name,
  ]
}
