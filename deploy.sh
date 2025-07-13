#!/bin/bash

# ProofParcel Deployment Script
# This script helps deploy the ProofParcel application to the Internet Computer

set -e

echo "🚀 ProofParcel Deployment Script"
echo "=================================="

# Check if dfx is installed
if ! command -v dfx &> /dev/null; then
    echo "❌ DFX is not installed. Please install it first:"
    echo "   https://internetcomputer.org/docs/current/developer-docs/setup/install/"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo "❌ Please run this script from the project root directory"
    exit 1
fi

# Function to check if local replica is running
check_replica() {
    if curl -s http://localhost:4943 > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to start local replica
start_replica() {
    echo "🔄 Starting local replica..."
    dfx start --background --clean
    echo "✅ Local replica started"
}

# Function to deploy to local network
deploy_local() {
    echo "🔄 Deploying to local network..."
    
    # Generate candid types
    echo "📝 Generating candid types..."
    dfx generate proofparcel_backend
    
    # Build and deploy
    echo "🔨 Building and deploying canisters..."
    dfx deploy
    
    echo "✅ Deployment completed!"
    echo ""
    echo "🌐 Application URLs:"
    echo "   Frontend: http://localhost:3000"
    echo "   Canister: http://localhost:4943"
    echo ""
    echo "📋 Next steps:"
    echo "   1. Start the frontend: cd src/proofparcel_frontend && npm run start"
    echo "   2. Open http://localhost:3000 in your browser"
    echo "   3. Create your first delivery!"
}

# Function to deploy to testnet
deploy_testnet() {
    echo "🔄 Deploying to testnet..."
    
    # Check if user is authenticated
    if ! dfx identity whoami &> /dev/null; then
        echo "❌ Please authenticate with dfx first:"
        echo "   dfx identity new my_identity"
        echo "   dfx identity use my_identity"
        echo "   dfx ledger create-canister"
        exit 1
    fi
    
    # Generate candid types
    echo "📝 Generating candid types..."
    dfx generate proofparcel_backend
    
    # Build and deploy to testnet
    echo "🔨 Building and deploying to testnet..."
    dfx deploy --network testnet
    
    echo "✅ Testnet deployment completed!"
    echo ""
    echo "🌐 Application URL:"
    echo "   https://$(dfx canister --network testnet id proofparcel_frontend).ic0.app"
}

# Function to deploy to mainnet
deploy_mainnet() {
    echo "⚠️  WARNING: You are about to deploy to MAINNET!"
    echo "   This will use real ICP tokens and deploy to the live network."
    echo ""
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Deployment cancelled"
        exit 1
    fi
    
    # Check if user is authenticated
    if ! dfx identity whoami &> /dev/null; then
        echo "❌ Please authenticate with dfx first:"
        echo "   dfx identity new my_identity"
        echo "   dfx identity use my_identity"
        echo "   dfx ledger create-canister"
        exit 1
    fi
    
    # Generate candid types
    echo "📝 Generating candid types..."
    dfx generate proofparcel_backend
    
    # Build and deploy to mainnet
    echo "🔨 Building and deploying to mainnet..."
    dfx deploy --network ic
    
    echo "✅ Mainnet deployment completed!"
    echo ""
    echo "🌐 Application URL:"
    echo "   https://$(dfx canister --network ic id proofparcel_frontend).ic0.app"
}

# Function to install dependencies
install_deps() {
    echo "📦 Installing dependencies..."
    
    # Install frontend dependencies
    if [ -d "src/proofparcel_frontend" ]; then
        echo "   Installing frontend dependencies..."
        cd src/proofparcel_frontend
        npm install
        cd ../..
    fi
    
    echo "✅ Dependencies installed"
}

# Function to run tests
run_tests() {
    echo "🧪 Running tests..."
    
    # Run backend tests
    echo "   Running backend tests..."
    dfx test
    
    # Run frontend tests if available
    if [ -d "src/proofparcel_frontend" ]; then
        echo "   Running frontend tests..."
        cd src/proofparcel_frontend
        if npm run test &> /dev/null; then
            npm run test
        else
            echo "   No frontend tests configured"
        fi
        cd ../..
    fi
    
    echo "✅ Tests completed"
}

# Function to show help
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  local     Deploy to local network (default)"
    echo "  testnet   Deploy to testnet"
    echo "  mainnet   Deploy to mainnet"
    echo "  install   Install dependencies"
    echo "  test      Run tests"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 local     # Deploy to local network"
    echo "  $0 testnet   # Deploy to testnet"
    echo "  $0 install   # Install dependencies"
}

# Main script logic
case "${1:-local}" in
    "local")
        if ! check_replica; then
            start_replica
        fi
        deploy_local
        ;;
    "testnet")
        deploy_testnet
        ;;
    "mainnet")
        deploy_mainnet
        ;;
    "install")
        install_deps
        ;;
    "test")
        run_tests
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac 