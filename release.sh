if ! nix-shell --help &> /dev/null
then
    echo "nix-shell could not be found! Are you sure it is installed correctly?"
    exit
fi

echo "Creating three releases of Generic-Expression inside ./release"

[ ! -d "./release" ] && mkdir "./release"

echo "Create release ..."

# Backup test dna config file
cp ./hc-dna/workdir/dna.yaml ./hc-dna/workdir/dna_origin.yaml

# Get new dna.yaml with correct props & build language
cp ./hc-dna/workdir/shortform_dna.yaml ./hc-dna/workdir/dna.yaml
npm run build

# Check if shortform directory exists, if not create
[ ! -d "./release/shortform" ] && mkdir "./release/shortform"

# Revert test dna config file
mv ./hc-dna/workdir/dna_origin.yaml ./hc-dna/workdir/dna.yaml

# Copy the build files to the release dir
cp ./build/bundle.js ./release/shortform/bundle.js
cp ./hc-dna/workdir/generic-expression.dna ./release/shortform/generic-expression.dna

cd ./release/shortform && zip -j -r ../shortform.zip ./* && cd -