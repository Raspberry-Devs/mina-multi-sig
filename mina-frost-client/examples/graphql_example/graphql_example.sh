
SCRIPT_DIR=$(dirname "$0")
GENERATED_DIR="$SCRIPT_DIR/generated"
HELPERS_DIR=$(dirname "$0")/../helpers
SIGNATURE_PATH="$SCRIPT_DIR/../signing_example/generated/signature.json"
OUTPUT_PATH="$GENERATED_DIR/out.json"

cd "$SCRIPT_DIR"
source "$HELPERS_DIR/use_frost_client.sh"

# Clean generated directory
rm -rf "$GENERATED_DIR"
mkdir -p "$GENERATED_DIR"

# If no signature in signing example prompt user to run it first
if [ ! -f "$SIGNATURE_PATH" ]; then
  echo "Please run the signing example first: ./signing_example/signing_example.sh"
  exit 1
fi

# Run the GraphQL example
use_frost_client graphql-build -i "$SIGNATURE_PATH" -o "$OUTPUT_PATH"
