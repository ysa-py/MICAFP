diff --git a/path/to/script.sh b/path/to/script.sh
@@
-cd "$(dirname "$0")/../.."   # repo root
+cd "$(dirname "$0")/../.." || exit   # repo root