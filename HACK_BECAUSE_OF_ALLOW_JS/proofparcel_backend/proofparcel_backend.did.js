export const idlFactory = ({ IDL }) => {
    const ConfirmDeliveryRequest = IDL.Record({
        'otp': IDL.Text,
        'delivery_id': IDL.Text,
    });
    const CreateDeliveryRequest = IDL.Record({
        'description': IDL.Text,
        'buyer': IDL.Principal,
        'amount': IDL.Nat64,
    });
    const DeliveryStatus = IDL.Variant({
        'InTransit': IDL.Null,
        'Delivered': IDL.Null,
        'EscrowReleased': IDL.Null,
        'Confirmed': IDL.Null,
        'Cancelled': IDL.Null,
        'Pending': IDL.Null,
    });
    const Delivery = IDL.Record({
        'id': IDL.Text,
        'otp': IDL.Opt(IDL.Text),
        'status': DeliveryStatus,
        'description': IDL.Text,
        'created_at': IDL.Nat64,
        'seller': IDL.Principal,
        'otp_expires_at': IDL.Opt(IDL.Nat64),
        'escrow_released_at': IDL.Opt(IDL.Nat64),
        'buyer': IDL.Principal,
        'amount': IDL.Nat64,
        'confirmed_at': IDL.Opt(IDL.Nat64),
    });
    const DeliveryNFT = IDL.Record({
        'id': IDL.Text,
        'delivery_id': IDL.Text,
        'owner': IDL.Principal,
        'metadata': IDL.Text,
        'minted_at': IDL.Nat64,
    });
    return IDL.Service({
        'confirm_delivery': IDL.Func([ConfirmDeliveryRequest], [IDL.Variant({ 'Ok': IDL.Text, 'Err': IDL.Text })], []),
        'create_delivery': IDL.Func([CreateDeliveryRequest], [IDL.Variant({ 'Ok': IDL.Text, 'Err': IDL.Text })], []),
        'generate_delivery_otp': IDL.Func([IDL.Text], [IDL.Variant({ 'Ok': IDL.Text, 'Err': IDL.Text })], []),
        'get_all_deliveries': IDL.Func([], [IDL.Vec(Delivery)], ['query']),
        'get_deliveries_by_buyer': IDL.Func([IDL.Principal], [IDL.Vec(Delivery)], ['query']),
        'get_deliveries_by_seller': IDL.Func([IDL.Principal], [IDL.Vec(Delivery)], ['query']),
        'get_delivery': IDL.Func([IDL.Text], [IDL.Opt(Delivery)], ['query']),
        'get_delivery_nft': IDL.Func([IDL.Text], [IDL.Opt(DeliveryNFT)], ['query']),
        'get_escrow_balance': IDL.Func([], [IDL.Nat64], ['query']),
        'get_nfts_by_owner': IDL.Func([IDL.Principal], [IDL.Vec(DeliveryNFT)], ['query']),
        'health_check': IDL.Func([], [IDL.Text], ['query']),
        'release_escrow': IDL.Func([IDL.Text], [IDL.Variant({ 'Ok': IDL.Null, 'Err': IDL.Text })], []),
        'start_delivery': IDL.Func([IDL.Text], [IDL.Variant({ 'Ok': IDL.Null, 'Err': IDL.Text })], []),
    });
};
export const init = ({ IDL }) => { return []; };
