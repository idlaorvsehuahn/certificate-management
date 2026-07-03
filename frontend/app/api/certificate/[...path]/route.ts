import { NextRequest, NextResponse } from 'next/server';
import { certificateClient } from '../../../../lib/api/api-client';

export async function GET(
  req: NextRequest,
  { params }: { params: Promise<{ path: string[] }> }
) {
  const { path: pathSegments } = await params;
  const path = pathSegments.join('/');
  const searchParams = req.nextUrl.searchParams.toString();
  const url = `/${path}${searchParams ? `?${searchParams}` : ''}`;
  try {
    const res = await certificateClient.get(url, { responseType: 'json' });
    return NextResponse.json(res.data);
  } catch (error: any) {
    return NextResponse.json(
      error.response?.data || { message: error.message },
      { status: error.response?.status || 500 }
    );
  }
}

export async function POST(
  req: NextRequest,
  { params }: { params: Promise<{ path: string[] }> }
) {
  const { path: pathSegments } = await params;
  const path = pathSegments.join('/');
  const body = await req.json().catch(() => ({}));
  console.log('API PROXY POST payload:', JSON.stringify(body));
  try {
    const res = await certificateClient.post(`/${path}`, body);
    return NextResponse.json(res.data);
  } catch (error: any) {
    return NextResponse.json(
      error.response?.data || { message: error.message },
      { status: error.response?.status || 500 }
    );
  }
}
