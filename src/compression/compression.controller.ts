import { CompressionAccessGuard } from '@app/compression/guards/compresion-access.guard';
import { Controller, Post, Req, UploadedFile, UseGuards, UseInterceptors } from '@nestjs/common';
import { Request } from 'express';

import axios from "axios";
import { FileInterceptor } from '@nestjs/platform-express';
// import { FormData } from 'formdata-node'; // Import FormData


@Controller('compression')
export class CompressionController {

  public constructor() {}

  // @UseGuards(CompressionAccessGuard)
  @Post('/')
  @UseInterceptors(FileInterceptor('file'))
  public async compressImage(@UploadedFile() file: Express.Multer.File) {
    if(!file) {
      return {
        success: false,
        message: "No files found"
      };
    }
	console.log("recieved request")
    try {
      const url = 'http://172.16.10.37:3004/upload';
      const form = new FormData();
      const blob = new Blob([file.buffer], { type: file.mimetype });

      form.append('file', blob, file.originalname);
  
      const response = await axios.post(url, form, {
        headers: {
          "Content-Type": "multipart/form-data"
        }
      });
      return {
        success: true,
        data: response.data
      };

    } catch (error) {
      return {
        success: false,
        message: "Failed to handle file",
        error
      };
    }
  }
}
