if ! nix-shell --help &> /dev/null
then
    echo "nix-shell could not be found! Are you sure it is installed correctly?"
    exit
fi

echo "Creating three releases of Generic-Expression inside ./release"

[ ! -d "./release" ] && mkdir "./release"

echo "Create release ..."

#Get new dna.yaml with correct props & build language
npm run build

#Copy the build files to the release dir
cp ./build/bundle.js ./release/bundle.js
cp ./hc-dna/workdir/generic-expression.dna ./release/generic-expression.dna
