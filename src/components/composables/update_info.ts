export interface VersionComparation {
    current_version: string,
    version: string,
    note: string,
}

export type DownloadEvent =
  | {
      event: 'Started';
      data: {
        contentLength: number;
      };
    }
  | {
      event: 'Progress';
      data: {
        chunkLength: number;
      };
    }
  | {
      event: 'Finished';
      data: {
        downloadId: number;
      };
    };
